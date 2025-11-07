use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq)]
pub enum MessageStatus {
    Pending,        // 等待翻译
    Streaming,      // 流式传输中
    Success,        // 翻译成功
    Error(String),  // 翻译失败
}

#[derive(Clone, Debug)]
pub struct Message {
    pub id: usize,
    pub text: String,                  // 用户输入
    pub translation: String,           // 翻译结果（支持增量更新）
    pub translation_complete: bool,    // 翻译是否完成
    pub status: MessageStatus,
    pub timestamp: DateTime<Utc>,
    pub provider: String,
}

impl Message {
    pub fn new(id: usize, text: String, provider: String) -> Self {
        Self {
            id,
            text,
            translation: String::new(),
            translation_complete: false,
            status: MessageStatus::Pending,
            timestamp: Utc::now(),
            provider,
        }
    }

    pub fn start_streaming(&mut self) {
        self.status = MessageStatus::Streaming;
        self.translation.clear();
    }

    pub fn append_translation(&mut self, delta: &str) {
        self.translation.push_str(delta);
    }

    pub fn complete_translation(&mut self) {
        self.translation_complete = true;
        self.status = MessageStatus::Success;
    }

    pub fn set_error(&mut self, error: String) {
        self.status = MessageStatus::Error(error);
        self.translation_complete = true;
    }
}
