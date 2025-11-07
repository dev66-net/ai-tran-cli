pub mod openai;

use anyhow::Result;
use async_trait::async_trait;
use futures::Stream;
use std::pin::Pin;

pub type StreamingResponse = Pin<Box<dyn Stream<Item = Result<String>> + Send>>;

#[async_trait]
pub trait TranslationProvider: Send + Sync {
    /// 流式翻译（用于TUI模式）
    async fn translate_stream(&self, text: &str) -> Result<StreamingResponse>;

    /// 同步翻译（用于快速模式）
    async fn translate(&self, text: &str) -> Result<String>;

    /// Provider名称
    fn name(&self) -> &str;
}
