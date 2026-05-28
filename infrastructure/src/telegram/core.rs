use std::sync::Arc;
use teloxide::prelude::*;
use teloxide::repl;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot::Sender as OneshotSender;
use tokio::sync::{Notify, oneshot};
use tracing::{info, warn};

use crate::config::AddUserResult;

#[allow(clippy::type_complexity)]
pub async fn register_allowed_users(
    token: impl Into<String>,
    tx: Sender<(String, ChatId, OneshotSender<(AddUserResult, Arc<Notify>)>)>,
) -> ResponseResult<()> {
    let bot = Bot::new(token.into());
    info!("Telegram registration listener started");

    repl(bot, move |bot: Bot, msg: Message| {
        let tx = tx.clone();
        let name = msg.text().map(str::to_owned);

        async move {
            info!("Received registration message from {}", msg.chat.id);
            let Some(name) = name else {
                return respond(());
            };

            let (response_tx, response_rx) = oneshot::channel();
            if tx.send((name, msg.chat.id, response_tx)).await.is_err() {
                warn!("Registration channel closed");
                return respond(());
            }

            match response_rx.await {
                Ok((result, sent_notify)) => {
                    let message = match result {
                        AddUserResult::Added => "✅ You have been added to FlowPilot.",
                        AddUserResult::NameAlreadyExists => "⚠️ This name is already used.",
                        AddUserResult::ChatIdAlreadyExists => "⚠️ You are already registered.",
                    };

                    if let Err(e) = bot.send_message(msg.chat.id, message).await {
                        warn!(
                            "Failed to send Telegram response to {}: {:?}",
                            msg.chat.id, e
                        );
                    } else {
                        info!("Sent Telegram response to {}", msg.chat.id);
                    }

                    sent_notify.notify_one();
                }
                Err(_) => {
                    warn!("Failed to receive response for chat {}", msg.chat.id);
                }
            }

            respond(())
        }
    })
    .await;

    Ok(())
}
