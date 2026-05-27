use teloxide::prelude::*;
use teloxide::repl;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot::{self, Sender as OneshotSender};
use tracing::{info, warn};

pub async fn register_allowed_users(
    token: impl Into<String>,
    tx: Sender<(ChatId, OneshotSender<bool>)>,
) -> ResponseResult<()> {
    let bot = Bot::new(token.into());
    info!("Telegram registration listener started");

    repl(bot, move |bot: Bot, msg: Message| {
        let tx = tx.clone();

        async move {
            info!("Received registration message from {}", msg.chat.id);
            let (response_tx, response_rx) = oneshot::channel();

            if tx.send((msg.chat.id, response_tx)).await.is_err() {
                warn!("Registration channel closed");
                return respond(());
            }

            let message = match response_rx.await {
                Ok(true) => "✅ You have been added to FlowPilot.",
                Ok(false) => "⚠️ You are already registered.",
                Err(_) => "❌ Internal error.",
            };

            if bot.send_message(msg.chat.id, message).await.is_err() {
                warn!("Failed to send Telegram response to {}", msg.chat.id);
            }

            respond(())
        }
    })
    .await;

    Ok(())
}
