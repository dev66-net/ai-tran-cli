use crate::app::App;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let mode_text = format!("Mode: {}", app.display_mode.to_string());
    let shortcuts: Vec<(&str, &str)> = vec![
        ("Enter", "Send"),
        ("TAB", &mode_text),
        ("Ctrl+Y", "Copy Latest"),
        ("1-9", "Copy #N"),
        ("Ctrl+C", "Clear"),
        ("ESC", "Quit"),
    ];

    let mut spans = Vec::new();

    for (i, (key, desc)) in shortcuts.iter().enumerate() {
        if i > 0 {
            spans.push(Span::raw(" | "));
        }
        spans.push(Span::styled(
            key.to_string(),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ));
        spans.push(Span::raw(": "));
        spans.push(Span::raw(desc.to_string()));
    }

    // Show notification if any
    if let Some(notification) = app.get_notification() {
        spans.push(Span::raw(" | "));
        spans.push(Span::styled(
            format!("ℹ {}", notification),
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ));
    }

    let paragraph = Paragraph::new(Line::from(spans))
        .style(Style::default().bg(Color::Black).fg(Color::White));

    frame.render_widget(paragraph, area);
}
