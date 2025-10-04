use std::process::Command;
use std::thread;
use std::time::Duration;

const SHOW_THRESHOLD: i64 = 5; // Show waybar when cursor Y <= threshold
const HIDE_THRESHOLD: i64 = 45; // Hide waybar when cursor Y > threshold
const POLL_INTERVAL_MS: u64 = 100; // Check cursor position every 100ms

fn get_cursor_y() -> Option<i64> {
    let output = Command::new("hyprctl")
        .args(["cursorpos", "-j"])
        .output()
        .ok()?;

    let json_str = String::from_utf8(output.stdout).ok()?;
    let parsed: serde_json::Value = serde_json::from_str(&json_str).ok()?;

    parsed["y"].as_i64()
}

fn toggle_waybar() {
    let _ = Command::new("pkill")
        .args(["-x", "-SIGUSR1", ".waybar-wrapped"])
        .output();
}

fn main() {
    // true = visible, false = hidden
    let mut bar_visible = true;

    loop {
        if let Some(y) = get_cursor_y() {
            let should_be_visible = if y <= SHOW_THRESHOLD {
                true
            } else if y > HIDE_THRESHOLD {
                false
            } else {
                bar_visible // Hysteresis zone
            };

            if should_be_visible != bar_visible {
                toggle_waybar();
                bar_visible = should_be_visible;
            }
        }

        thread::sleep(Duration::from_millis(POLL_INTERVAL_MS));
    }
}
