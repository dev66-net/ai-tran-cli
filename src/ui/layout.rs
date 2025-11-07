use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

use super::{chat, input, statusbar};

pub fn render_ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(5),     // Chat area
            Constraint::Length(3),  // Input box
            Constraint::Length(1),  // Status bar
        ])
        .split(frame.size());

    render_chat_area(frame, app, chunks[0]);
    render_input_area(frame, app, chunks[1]);
    render_status_bar(frame, app, chunks[2]);
}

fn render_chat_area(frame: &mut Frame, app: &App, area: Rect) {
    chat::render(frame, app, area);
}

fn render_input_area(frame: &mut Frame, app: &App, area: Rect) {
    input::render(frame, app, area);
}

fn render_status_bar(frame: &mut Frame, app: &App, area: Rect) {
    statusbar::render(frame, app, area);
}
