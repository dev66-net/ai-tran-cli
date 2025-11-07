use super::Message;
use std::time::Instant;
use tokio::sync::mpsc;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DisplayMode {
    TranslationOnly,  // 仅翻译
    Bilingual,        // 双语对照
    OriginalOnly,     // 仅原文
}

impl DisplayMode {
    pub fn next(&self) -> Self {
        match self {
            Self::TranslationOnly => Self::Bilingual,
            Self::Bilingual => Self::TranslationOnly,
            Self::OriginalOnly => Self::TranslationOnly,
            // 两种模式循环切换
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            Self::TranslationOnly => "Trans",
            Self::Bilingual => "Both",
            Self::OriginalOnly => "Orig",
        }
    }
}

pub enum AppMessage {
    TranslationDelta(usize, String),    // (msg_id, delta)
    TranslationComplete(usize),         // msg_id
    TranslationError(usize, String),    // (msg_id, error)
}

pub struct App {
    pub messages: Vec<Message>,
    pub input: String,
    pub scroll: usize,
    pub should_quit: bool,
    pub display_mode: DisplayMode,
    pub notification: Option<(String, Instant)>,
    pub next_msg_id: usize,
    pub provider_name: String,
    pub tx: mpsc::UnboundedSender<AppMessage>,
    pub rx: mpsc::UnboundedReceiver<AppMessage>,
}

impl App {
    pub fn new(provider_name: String) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();

        Self {
            messages: Vec::new(),
            input: String::new(),
            scroll: 0,
            should_quit: false,
            display_mode: DisplayMode::Bilingual,
            notification: None,
            next_msg_id: 0,
            provider_name,
            tx,
            rx,
        }
    }

    pub fn toggle_display_mode(&mut self) {
        self.display_mode = self.display_mode.next();
        self.show_notification(format!("Display mode: {}", self.display_mode.to_string()));
    }

    pub fn show_notification(&mut self, msg: impl Into<String>) {
        self.notification = Some((msg.into(), Instant::now()));
    }

    pub fn get_notification(&self) -> Option<&str> {
        if let Some((msg, time)) = &self.notification {
            // 3秒后自动消失
            if time.elapsed().as_secs() < 3 {
                return Some(msg);
            }
        }
        None
    }

    pub fn create_message(&mut self, text: String) -> Message {
        let id = self.next_msg_id;
        self.next_msg_id += 1;
        Message::new(id, text, self.provider_name.clone())
    }

    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
        self.scroll_to_bottom();
    }

    pub fn scroll_to_bottom(&mut self) {
        self.scroll = self.messages.len().saturating_sub(1);
    }

    pub fn clear_history(&mut self) {
        self.messages.clear();
        self.scroll = 0;
        self.show_notification("History cleared");
    }

    pub fn handle_translation_update(&mut self, msg: AppMessage) {
        match msg {
            AppMessage::TranslationDelta(id, delta) => {
                if let Some(message) = self.messages.iter_mut().find(|m| m.id == id) {
                    message.append_translation(&delta);
                }
            }
            AppMessage::TranslationComplete(id) => {
                if let Some(message) = self.messages.iter_mut().find(|m| m.id == id) {
                    message.complete_translation();
                }
            }
            AppMessage::TranslationError(id, error) => {
                if let Some(message) = self.messages.iter_mut().find(|m| m.id == id) {
                    message.set_error(error);
                }
            }
        }
    }

    pub fn get_latest_translation(&self) -> Option<String> {
        self.messages
            .last()
            .filter(|m| m.translation_complete)
            .map(|m| m.translation.clone())
    }

    pub fn get_translation_by_index(&self, index: usize) -> Option<String> {
        self.messages
            .get(index)
            .filter(|m| m.translation_complete)
            .map(|m| m.translation.clone())
    }

    pub fn get_all_translations(&self) -> Vec<String> {
        self.messages
            .iter()
            .filter(|m| m.translation_complete)
            .map(|m| m.translation.clone())
            .collect()
    }
}
