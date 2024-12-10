// src/screen.rs

use std::process::Command;
use std::path::Path;

pub fn record_screen(seconds: u32) -> Result<String, String> {
    let file_path = "screen_record.mp4";
    let status = Command::new("ffmpeg")
        .args(&["-video_size", "1920x1080", "-framerate", "30", "-f", "x11grab", "-i", ":0.0"])
        .arg("-t")
        .arg(seconds.to_string())
        .arg(file_path)
        .status();

    if status.is_ok() && Path::new(file_path).exists() {
        Ok(file_path.to_string())
    } else {
        Err("Failed to record screen.".to_string())
    }
}
