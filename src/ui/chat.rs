use crate::app::{App, DisplayMode, MessageStatus};
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let mut lines: Vec<Line> = Vec::new();

    // Calculate visible messages based on scroll
    let visible_count = (area.height.saturating_sub(2)) as usize; // -2 for borders
    let total_messages = app.messages.len();
    let start_idx = if total_messages > visible_count {
        app.scroll.min(total_messages.saturating_sub(visible_count))
    } else {
        0
    };

    for (idx, message) in app.messages.iter().enumerate().skip(start_idx) {
        // Add separator between messages
        if idx > start_idx {
            lines.push(Line::from(""));
        }

        // Message number and status indicator
        let status_indicator = match &message.status {
            MessageStatus::Pending => "⏳",
            MessageStatus::Streaming => "⚡",
            MessageStatus::Success => "✓",
            MessageStatus::Error(_) => "✗",
        };

        let number_style = Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD);

        lines.push(Line::from(vec![
            Span::styled(format!("[{}] ", idx + 1), number_style),
            Span::raw(status_indicator),
            Span::raw(" "),
            Span::styled(
                message.timestamp.format("%H:%M:%S").to_string(),
                Style::default().fg(Color::DarkGray),
            ),
        ]));

        // Render based on display mode
        match app.display_mode {
            DisplayMode::TranslationOnly => {
                render_translation_only(&mut lines, message);
            }
            DisplayMode::Bilingual => {
                render_bilingual(&mut lines, message);
            }
            DisplayMode::OriginalOnly => {
                render_original_only(&mut lines, message);
            }
        }

        // Show error if any
        if let MessageStatus::Error(ref err) = message.status {
            lines.push(Line::from(vec![
                Span::styled("Error: ", Style::default().fg(Color::Red)),
                Span::styled(err.clone(), Style::default().fg(Color::Red)),
            ]));
        }
    }

    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" Translation History ({}) ", app.provider_name))
        .border_style(Style::default().fg(Color::White));

    let paragraph = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false });

    frame.render_widget(paragraph, area);
}

fn render_translation_only(lines: &mut Vec<Line>, message: &crate::app::Message) {
    if message.translation.is_empty() && message.status == MessageStatus::Streaming {
        lines.push(Line::from(Span::styled(
            "Translating...",
            Style::default().fg(Color::Yellow),
        )));
    } else {
        let mut translation = message.translation.clone();
        if message.status == MessageStatus::Streaming {
            translation.push('▊'); // Streaming cursor
        }
        lines.push(Line::from(Span::styled(
            translation,
            Style::default().fg(Color::Green),
        )));
    }
}

fn render_bilingual(lines: &mut Vec<Line>, message: &crate::app::Message) {
    // Original text
    lines.push(Line::from(vec![
        Span::styled("  Original: ", Style::default().fg(Color::Blue)),
        Span::raw(message.text.clone()),
    ]));

    // Translation
    if message.translation.is_empty() && message.status == MessageStatus::Streaming {
        lines.push(Line::from(vec![
            Span::styled("  Translation: ", Style::default().fg(Color::Green)),
            Span::styled("...", Style::default().fg(Color::Yellow)),
        ]));
    } else {
        let mut translation = message.translation.clone();
        if message.status == MessageStatus::Streaming {
            translation.push('▊'); // Streaming cursor
        }
        lines.push(Line::from(vec![
            Span::styled("  Translation: ", Style::default().fg(Color::Green)),
            Span::raw(translation),
        ]));
    }
}

fn render_original_only(lines: &mut Vec<Line>, message: &crate::app::Message) {
    lines.push(Line::from(Span::styled(
        message.text.clone(),
        Style::default().fg(Color::Blue),
    )));
}
