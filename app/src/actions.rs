use tokio::spawn;
use tokio::sync::mpsc;
use tracing::info;

use crate::cli::DeleteTarget;
use crate::error::AppError;
use infrastructure::config::Config;
use infrastructure::filesystem::Paths;
use infrastructure::telegram::register_allowed_users;

pub(crate) fn init(paths: &Paths, token: &str) -> Result<(), AppError> {
    if paths.config_exists() {
        return Err(AppError::AlreadyInitialized);
    }

    let mut config = Config::default();
    config.set_telegram_token(token);

    paths.save_config(&config)?;
    info!("FlowPilot initialized");

    Ok(())
}

pub(crate) fn delete(paths: &Paths, target: &DeleteTarget) -> Result<(), AppError> {
    match target {
        DeleteTarget::All => {
            paths.delete_all()?;
        }
        DeleteTarget::Config => {
            paths.delete_config()?;
        }
    }

    info!("FlowPilot delete command completed");

    Ok(())
}

pub(crate) async fn add_user(paths: &Paths, count: usize) -> Result<(), AppError> {
    if count == 0 {
        return Err(AppError::InvalidUserCount);
    }

    let mut config = paths.load_config()?;
    let token = config.telegram_token().to_string();
    let (tx, mut rx) = mpsc::channel(count);

    let handle = spawn(register_allowed_users(token, tx));
    info!("Waiting for {} Telegram user(s)", count);

    let mut added_count = 0;

    while added_count < count {
        let (chat_id, response_tx) = rx.recv().await.ok_or(AppError::RegistrationChannelClosed)?;
        let added = config.add_allowed_chat_id(chat_id);

        if added {
            added_count += 1;

            info!("Added user {} ({}/{})", chat_id, added_count, count);
            paths.save_config(&config)?;
        }

        response_tx
            .send(added)
            .map_err(|_| AppError::RegistrationResponseSendFailed)?;
    }

    handle.abort();
    info!("User registration completed");
    Ok(())
}
