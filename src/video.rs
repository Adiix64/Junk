// src/video.rs

use std::process::Command;
use std::path::Path;

pub fn record_video(seconds: u32) -> Result<String, String> {
    let file_path = "video_record.mp4";
    let status = Command::new("ffmpeg")
        .args(&["-f", "v4l2", "-i", "/dev/video0"])
        .arg("-t")
        .arg(seconds.to_string())
        .arg(file_path)
        .status();

    if status.is_ok() && Path::new(file_path).exists() {
        Ok(file_path.to_string())
    } else {
        Err("Failed to record video.".to_string())
    }
}
