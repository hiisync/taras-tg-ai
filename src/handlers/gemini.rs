use crate::{config::secrets::GEMINI_API_KEY, utils::requests::GeminiRequest};
use log::info;
use serde::Deserialize;
use teloxide::prelude::*;

#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}

#[derive(Deserialize)]
struct Candidate {
    content: Content,
}

#[derive(Deserialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Deserialize)]
struct Part {
    text: String,
}

pub async fn handle_message(
    message: Message,
    bot: Bot,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Логування отриманого повідомлення
    info!("Отримано повідомлення: {:?}", message.text());

    // Створюємо запит до API
    let request = GeminiRequest::new(
        GEMINI_API_KEY.to_string(),
        message.text().unwrap_or_default().to_owned(),
    );

    // Відправляємо запит і перевіряємо відповідь
    let response = request.send().await?;

    // Десеріалізуємо JSON у структуру
    let json_response: GeminiResponse = serde_json::from_str(&response.response)?;

    let text = &json_response.candidates[0].content.parts[0].text;
    let keywords = ["Тарас", "Бот", "Taras"];

    // Логування для діагностики
    info!("Key words to check: {:?}", keywords);
    info!("Answer text: {}", text);

    // Перевіряємо, чи одне зі слів є в повідомленні
    if keywords.iter().any(|&keyword| {
        message
            .text()
            .unwrap_or_default()
            .to_lowercase()
            .contains(&keyword.to_lowercase())
    }) {
        // Відправляємо відповідь користувачу
        info!("Keyword is found, sending a message.");
        bot.send_message(message.chat.id, text).await?;
    } else {
        info!("Keyword is not found");
    }

    Ok(())
}
