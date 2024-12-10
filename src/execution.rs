// src/execution.rs

use std::process::Command;

pub fn execute_command(command: &str) -> String {
    match Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
    {
        Ok(output) => {
            if output.status.success() {
                String::from_utf8_lossy(&output.stdout).to_string()
            } else {
                String::from_utf8_lossy(&output.stderr).to_string()
            }
        }
        Err(e) => format!("Error: {}", e),
    }
}
