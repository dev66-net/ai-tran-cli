mod app;
mod events;
mod providers;
mod ui;

use anyhow::Result;
use app::App;
use clap::Parser;
use crossterm::{
    event::{self, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use providers::{openai::OpenAIProvider, TranslationProvider};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io::{self, IsTerminal, Read}, sync::Arc, time::Duration};

#[derive(Parser, Debug)]
#[command(name = "ai-tran-cli")]
#[command(about = "AI Translation CLI - A fast and beautiful translation tool", long_about = None)]
struct Args {
    /// Quick mode: output translation and exit immediately
    #[arg(short = 'q', long = "quick")]
    quick: bool,

    /// Verbose mode: print detailed debug information
    #[arg(short = 'v', long = "verbose")]
    verbose: bool,

    /// Text to translate (optional, can also use stdin)
    text: Option<String>,
}

// Global verbose flag
static VERBOSE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

fn verbose_log(msg: impl AsRef<str>) {
    if VERBOSE.load(std::sync::atomic::Ordering::Relaxed) {
        eprintln!("[VERBOSE] {}", msg.as_ref());
    }
}

fn mask_sensitive(s: &str, prefix: usize, suffix: usize) -> String {
    if s.len() <= prefix + suffix {
        return "*".repeat(s.len());
    }
    let visible_len = prefix + suffix;
    let mask_len = s.len() - visible_len;
    format!(
        "{}{}{}",
        &s[..prefix],
        "*".repeat(mask_len),
        &s[s.len() - suffix..]
    )
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command-line arguments
    let args = Args::parse();

    // Set global verbose flag
    VERBOSE.store(args.verbose, std::sync::atomic::Ordering::Relaxed);

    // Load environment variables
    verbose_log("Loading environment variables from .env file");
    dotenv::dotenv().ok();

    // Initialize provider
    verbose_log("Initializing OpenAI provider");
    let provider = Arc::new(OpenAIProvider::from_env(args.verbose)?);

    if args.verbose {
        verbose_log(format!("Provider: {}", provider.name()));
        verbose_log(format!("API Base: {}", provider.get_api_base()));
        verbose_log(format!("Model: {}", provider.get_model()));
        verbose_log(format!("API Key: {}", mask_sensitive(provider.get_api_key(), 7, 4)));
        verbose_log(format!("Target Language: {}", provider.get_target_language()));
    }

    // Get input text from stdin or argument
    let input_text = get_input_text(&args)?;

    // Handle quick mode
    if args.quick {
        if let Some(text) = input_text {
            match provider.translate(&text).await {
                Ok(translation) => {
                    println!("{}", translation);
                }
                Err(e) => {
                    eprintln!("Translation error: {}", e);
                    std::process::exit(1);
                }
            }
        } else {
            eprintln!("Error: No input text provided. Use stdin or provide text as argument.");
            std::process::exit(1);
        }
        return Ok(());
    }

    // TUI mode
    let provider_name = provider.name().to_string();
    let mut app = App::new(provider_name);

    // If input from pipe, auto-commit it
    if let Some(text) = input_text {
        let mut message = app.create_message(text.clone());
        message.start_streaming();
        let msg_id = message.id;
        app.add_message(message);

        // Start translation task
        let tx = app.tx.clone();
        let provider_clone = Arc::clone(&provider);

        tokio::spawn(async move {
            match provider_clone.translate_stream(&text).await {
                Ok(mut stream) => {
                    use futures::StreamExt;
                    while let Some(result) = stream.next().await {
                        match result {
                            Ok(delta) => {
                                if !delta.is_empty() {
                                    let _ = tx.send(app::AppMessage::TranslationDelta(msg_id, delta));
                                }
                            }
                            Err(e) => {
                                let _ = tx.send(app::AppMessage::TranslationError(msg_id, e.to_string()));
                                return;
                            }
                        }
                    }
                    let _ = tx.send(app::AppMessage::TranslationComplete(msg_id));
                }
                Err(e) => {
                    let _ = tx.send(app::AppMessage::TranslationError(msg_id, e.to_string()));
                }
            }
        });
    }

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run the app
    let result = run_app(&mut terminal, &mut app, provider).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    // Print any errors
    if let Err(err) = result {
        eprintln!("Error: {}", err);
    }

    Ok(())
}

fn get_input_text(args: &Args) -> Result<Option<String>> {
    // Check if text provided as argument
    if let Some(ref text) = args.text {
        return Ok(Some(text.clone()));
    }

    // Check if stdin is piped (not a terminal)
    if !io::stdin().is_terminal() {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input)?;
        let trimmed = input.trim();
        if !trimmed.is_empty() {
            return Ok(Some(trimmed.to_string()));
        }
    }

    Ok(None)
}

async fn run_app<B: ratatui::backend::Backend, P: TranslationProvider + 'static>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    provider: Arc<P>,
) -> Result<()> {
    loop {
        // Draw UI
        terminal.draw(|f| ui::render_ui(f, app))?;

        // Handle translation updates (non-blocking)
        while let Ok(msg) = app.rx.try_recv() {
            app.handle_translation_update(msg);
        }

        // Handle keyboard events (with timeout)
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                events::handle_key_event(key, app, Arc::clone(&provider)).await?;
            }
        }

        // Check if should quit
        if app.should_quit {
            break;
        }
    }

    Ok(())
}
