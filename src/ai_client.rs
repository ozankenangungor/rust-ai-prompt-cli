use std::marker::PhantomData;

use reqwest::{IntoUrl, Url};
use serde::{Deserialize, Serialize};

const MODEL_NAME: &str = "Llama 3.2 1B Instruct";

pub struct Gpt4AllClient<U: IntoUrl> {
    url: Url,
    client: reqwest::Client,
    historical_messages: Vec<ChatRequestMessage>,
    _phantom_url: PhantomData<U>,
}

impl<U: IntoUrl> Gpt4AllClient<U> {
    pub fn new(url: U) -> Result<Self, reqwest::Error> {
        Ok(Self {
            client: reqwest::ClientBuilder::new().build()?,
            historical_messages: vec![],
            url: url.into_url()?,
            _phantom_url: PhantomData,
        })
    }

    pub fn send(&mut self, message: String) {}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatRequestMessage>,
    pub max_tokens: u64,
    pub temperature: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChatRequestMessageRole {
    System,
    User,
    Assistant,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatRequestMessage {
    pub role: ChatRequestMessageRole,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatResponse {
    pub choices: Vec<ChatResponseChoice>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatResponseChoice {
    pub nessages: ChatRequestMessage,
}
