// src/audio.rs

use std::process::Command;
use std::path::Path;

pub fn record_audio(seconds: u32) -> Result<String, String> {
    let file_path = "audio_record.wav";
    let status = Command::new("arecord")
        .arg("-d")
        .arg(seconds.to_string())
        .arg("-f")
        .arg("cd")
        .arg(file_path)
        .status();

    if status.is_ok() && Path::new(file_path).exists() {
        Ok(file_path.to_string())
    } else {
        Err("Failed to record audio.".to_string())
    }
}
