use crate::app::App;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let input_text = if app.input.is_empty() {
        vec![Span::styled(
            "Type your text here and press Enter to translate...",
            Style::default().fg(Color::DarkGray),
        )]
    } else {
        vec![Span::raw(app.input.clone()), Span::raw("▊")]
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Input ")
        .border_style(Style::default().fg(Color::Cyan));

    let paragraph = Paragraph::new(Line::from(input_text)).block(block);

    frame.render_widget(paragraph, area);
}
