// src/screenshot.rs

use std::process::Command;
use std::path::Path;

pub fn capture_screenshot() -> Result<String, String> {
    let file_path = "screenshot.png";
    let status = Command::new("import")
        .arg("-window")
        .arg("root")
        .arg(file_path)
        .status();

    if status.is_ok() && Path::new(file_path).exists() {
        Ok(file_path.to_string())
    } else {
        Err("Failed to capture screenshot.".to_string())
    }
}
