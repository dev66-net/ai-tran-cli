use super::{StreamingResponse, TranslationProvider};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use eventsource_stream::Eventsource;
use futures::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<ChatMessage>,
    stream: bool,
}

#[derive(Debug, Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Debug, Deserialize)]
struct Message {
    content: String,
}

pub struct OpenAIProvider {
    client: Client,
    api_key: String,
    api_base: String,
    model: String,
    target_language: String,
    verbose: bool,
}

impl OpenAIProvider {
    pub fn new(api_key: String, api_base: String, model: String, target_language: String, verbose: bool) -> Self {
        Self {
            client: Client::new(),
            api_key,
            api_base,
            model,
            target_language,
            verbose,
        }
    }

    pub fn from_env(verbose: bool) -> Result<Self> {
        dotenv::dotenv().ok();

        let api_key = std::env::var("OPENAI_API_KEY")
            .map_err(|_| anyhow!("OPENAI_API_KEY not found in environment"))?;
        let api_base = std::env::var("OPENAI_API_BASE")
            .unwrap_or_else(|_| "https://api.openai.com/v1".to_string());
        let model = std::env::var("OPENAI_MODEL").unwrap_or_else(|_| "gpt-5-mini".to_string());
        let target_language =
            std::env::var("TARGET_LANGUAGE").unwrap_or_else(|_| "zh-CN".to_string());

        Ok(Self::new(api_key, api_base, model, target_language, verbose))
    }

    // Getter methods for verbose logging
    pub fn get_api_key(&self) -> &str {
        &self.api_key
    }

    pub fn get_api_base(&self) -> &str {
        &self.api_base
    }

    pub fn get_model(&self) -> &str {
        &self.model
    }

    pub fn get_target_language(&self) -> &str {
        &self.target_language
    }

    fn verbose_log(&self, msg: impl AsRef<str>) {
        if self.verbose {
            eprintln!("[VERBOSE] {}", msg.as_ref());
        }
    }

    fn mask_sensitive(&self, s: &str, prefix: usize, suffix: usize) -> String {
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

    fn create_prompt(&self, text: &str) -> String {
        format!(
            "You are a professional translator. Detect the language of the input text and translate it intelligently:
- If the input is in Chinese (简体中文/繁体中文), translate to English
- If the input is in English, translate to Chinese (Simplified Chinese, 简体中文)
- For other languages, translate to English

Only output the translation result, no explanations or additional text.

Input text:
{}",
            text
        )
    }
}

#[async_trait]
impl TranslationProvider for OpenAIProvider {
    async fn translate_stream(&self, text: &str) -> Result<StreamingResponse> {
        let request = ChatCompletionRequest {
            model: self.model.clone(),
            messages: vec![ChatMessage {
                role: "user".to_string(),
                content: self.create_prompt(text),
            }],
            stream: true,
        };

        let response = self
            .client
            .post(format!("{}/chat/completions", self.api_base))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await?;
            return Err(anyhow!("API request failed ({}): {}", status, text));
        }

        let stream = response
            .bytes_stream()
            .eventsource()
            .map(|event| match event {
                Ok(event) => {
                    if event.data == "[DONE]" {
                        return Ok(String::new());
                    }

                    let parsed: Result<Value, _> = serde_json::from_str(&event.data);
                    match parsed {
                        Ok(value) => {
                            let delta = value["choices"][0]["delta"]["content"]
                                .as_str()
                                .unwrap_or("")
                                .to_string();
                            Ok(delta)
                        }
                        Err(e) => Err(anyhow!("Failed to parse SSE event: {}", e)),
                    }
                }
                Err(e) => Err(anyhow!("Stream error: {}", e)),
            });

        Ok(Box::pin(stream))
    }

    async fn translate(&self, text: &str) -> Result<String> {
        self.verbose_log(format!("Translating text: {}", text));
        self.verbose_log("Auto-detecting language and translating...");

        let prompt = self.create_prompt(text);
        self.verbose_log(format!("Generated prompt: {}", prompt));

        let request = ChatCompletionRequest {
            model: self.model.clone(),
            messages: vec![ChatMessage {
                role: "user".to_string(),
                content: prompt,
            }],
            stream: false,
        };

        let url = format!("{}/chat/completions", self.api_base);
        self.verbose_log(format!("API URL: {}", url));
        self.verbose_log(format!("API Model: {}", self.model));
        self.verbose_log(format!("API Key: {}", self.mask_sensitive(&self.api_key, 7, 4)));

        self.verbose_log("Sending HTTP request...");
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        let status = response.status();
        self.verbose_log(format!("HTTP Status: {}", status));

        if !status.is_success() {
            let error_text = response.text().await?;
            self.verbose_log(format!("Error response body: {}", error_text));
            return Err(anyhow!("API request failed ({} {}): {}", status.as_u16(), status.canonical_reason().unwrap_or("Unknown"), error_text));
        }

        let response_text = response.text().await?;
        self.verbose_log(format!("Response body length: {} bytes", response_text.len()));
        if self.verbose {
            // Truncate long responses in verbose mode
            if response_text.len() > 500 {
                self.verbose_log(format!("Response preview: {}...", &response_text[..500]));
            } else {
                self.verbose_log(format!("Response body: {}", response_text));
            }
        }

        let completion: ChatCompletionResponse = serde_json::from_str(&response_text)
            .map_err(|e| anyhow!("Failed to parse response JSON: {}", e))?;

        let translation = completion
            .choices
            .get(0)
            .ok_or_else(|| anyhow!("No choices in response"))?
            .message
            .content
            .clone();

        self.verbose_log(format!("Translation result: {}", translation));
        Ok(translation.trim().to_string())
    }

    fn name(&self) -> &str {
        "OpenAI"
    }
}
