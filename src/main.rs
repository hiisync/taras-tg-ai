use config::secrets::TELEGRAM_API_TOKEN;
use dptree::endpoint;
use handlers::gemini::handle_message;
use shuttle_runtime::{Error, Service};
use std::net::SocketAddr;
use teloxide::prelude::*;

mod config;
mod handlers;
mod utils;

#[shuttle_runtime::main]
async fn shuttle_main() -> Result<BotService, Error> {
    log::info!("Starting dialogue bot...");

    // Create the bot instance
    let bot = Bot::new(TELEGRAM_API_TOKEN);

    // Return the service with the bot instance
    Ok(BotService { bot })
}

// Service struct that holds necessary components
struct BotService {
    bot: Bot,
}

#[shuttle_runtime::async_trait]
impl Service for BotService {
    async fn bind(self, _addr: SocketAddr) -> Result<(), Error> {
        // Start the dispatcher and handle incoming updates
        Dispatcher::builder(
            self.bot,
            Update::filter_message().branch(endpoint(handle_message)),
        )
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

        Ok(())
    }
}
