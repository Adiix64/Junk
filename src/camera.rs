// src/camera.rs

use std::process::Command;
use std::path::Path;

pub fn capture_photo() -> Result<String, String> {
    let file_path = "photo.jpg";
    let status = Command::new("fswebcam")
        .arg("-r")
        .arg("1280x720")
        .arg(file_path)
        .status();

    if status.is_ok() && Path::new(file_path).exists() {
        Ok(file_path.to_string())
    } else {
        Err("Failed to capture photo.".to_string())
    }
}
