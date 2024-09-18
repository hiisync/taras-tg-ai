use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use serde::Deserialize;
use serde_json::json;

pub struct GeminiRequest {
    client: Client,
    url: String,
    message: String,
}

#[derive(Deserialize)]
pub struct GeminiResponse {
    pub response: String,
}

impl GeminiRequest {
    pub fn new(api_key: String, message: String) -> Self {
        Self {
            client: Client::new(),
            url: format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}", api_key),
            message,
        }
    }

    pub async fn send(&self) -> Result<GeminiResponse, reqwest::Error> {
        let headers = {
            let mut headers = HeaderMap::new();
            headers.insert("Content-Type", HeaderValue::from_static("application/json"));
            headers
        };

        let body = json!({
            "contents": [
                {
                    "parts": [
                        {
                            "text": self.message
                        }
                    ]
                }
            ]
        });

        let response = self
            .client
            .post(&self.url)
            .headers(headers)
            .json(&body)
            .send()
            .await?;

        let response_json: serde_json::Value = response.json().await?;
        Ok(GeminiResponse {
            response: response_json.to_string(),
        })
    }
}
