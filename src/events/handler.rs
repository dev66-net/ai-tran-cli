use crate::app::{App, AppMessage};
use crate::providers::TranslationProvider;
use anyhow::Result;
use arboard::Clipboard;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use futures::StreamExt;
use std::sync::Arc;

pub async fn handle_key_event<P: TranslationProvider + 'static>(
    key: KeyEvent,
    app: &mut App,
    provider: Arc<P>,
) -> Result<()> {
    match key.code {
        // ESC: Quit
        KeyCode::Esc => {
            app.should_quit = true;
        }

        // Enter: Submit translation
        KeyCode::Enter => {
            if !app.input.is_empty() {
                let input_text = app.input.clone();
                app.input.clear();

                // Create message
                let mut message = app.create_message(input_text.clone());
                message.start_streaming();
                let msg_id = message.id;
                app.add_message(message);

                // Spawn translation task
                let tx = app.tx.clone();
                let provider_clone = Arc::clone(&provider);

                tokio::spawn(async move {
                    match provider_clone.translate_stream(&input_text).await {
                        Ok(mut stream) => {
                            while let Some(result) = stream.next().await {
                                match result {
                                    Ok(delta) => {
                                        if !delta.is_empty() {
                                            let _ = tx.send(AppMessage::TranslationDelta(msg_id, delta));
                                        }
                                    }
                                    Err(e) => {
                                        let _ = tx.send(AppMessage::TranslationError(
                                            msg_id,
                                            e.to_string(),
                                        ));
                                        return;
                                    }
                                }
                            }
                            let _ = tx.send(AppMessage::TranslationComplete(msg_id));
                        }
                        Err(e) => {
                            let _ = tx.send(AppMessage::TranslationError(msg_id, e.to_string()));
                        }
                    }
                });
            }
        }

        // TAB: Toggle display mode
        KeyCode::Tab => {
            app.toggle_display_mode();
        }

        // Backspace: Delete character
        KeyCode::Backspace => {
            app.input.pop();
        }

        // Ctrl+C: Clear history
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.clear_history();
        }

        // Ctrl+Y: Copy latest translation
        KeyCode::Char('y') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            if let Some(translation) = app.get_latest_translation() {
                if let Ok(mut clipboard) = Clipboard::new() {
                    if clipboard.set_text(&translation).is_ok() {
                        app.show_notification("Copied latest translation to clipboard");
                    }
                }
            } else {
                app.show_notification("No translation available to copy");
            }
        }

        // Ctrl+Shift+Y: Copy all translations
        KeyCode::Char('Y') if key.modifiers.contains(KeyModifiers::CONTROL | KeyModifiers::SHIFT) =>
        {
            let translations = app.get_all_translations();
            if !translations.is_empty() {
                let combined = translations.join("\n\n");
                if let Ok(mut clipboard) = Clipboard::new() {
                    if clipboard.set_text(&combined).is_ok() {
                        app.show_notification(format!(
                            "Copied {} translations to clipboard",
                            translations.len()
                        ));
                    }
                }
            } else {
                app.show_notification("No translations available to copy");
            }
        }

        // Number keys 1-9: Copy specific translation
        KeyCode::Char(c @ '1'..='9') => {
            let index = c.to_digit(10).unwrap() as usize - 1;
            if let Some(translation) = app.get_translation_by_index(index) {
                if let Ok(mut clipboard) = Clipboard::new() {
                    if clipboard.set_text(&translation).is_ok() {
                        app.show_notification(format!("Copied translation #{} to clipboard", index + 1));
                    }
                }
            } else {
                app.show_notification(format!("Translation #{} not found", index + 1));
            }
        }

        // Regular character input
        KeyCode::Char(c) => {
            app.input.push(c);
        }

        _ => {}
    }

    Ok(())
}
