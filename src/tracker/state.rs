// Local state file for the waybar module.
// The DB is still the source of truth for history -- this file exists purely so
// the bar can render a ticking timer without doing a network round trip to Neon
// every second.

use std::path::PathBuf;

// waybar module is configured with "signal": 10, so SIGRTMIN+10 tells it to
// re-run the script right now instead of waiting for the next 1s tick
const WAYBAR_SIGNAL: &str = "-SIGRTMIN+10";

// ~/.local/state/productivicli/active
fn state_path() -> Option<PathBuf> {
    // XDG_STATE_HOME if it's set, otherwise fall back to ~/.local/state
    let base = match std::env::var("XDG_STATE_HOME") {
        Ok(dir) => PathBuf::from(dir),
        Err(_) => PathBuf::from(std::env::var("HOME").ok()?).join(".local/state"),
    };

    Some(base.join("productivicli").join("active"))
}

// nudge waybar so the bar updates the instant we start/stop
fn signal_waybar() {
    // .ok() -- if pkill isn't installed or waybar isn't running we don't care,
    // the module will just catch up on its next interval tick
    let _ = std::process::Command::new("pkill")
        .args([WAYBAR_SIGNAL, "waybar"])
        .status();
}

pub fn write_active(task_name: &str, started_at: chrono::DateTime<chrono::Utc>) {
    let Some(path) = state_path() else {
        return; // no HOME? nothing sensible to do, the DB write already succeeded
    };

    // create_dir_all is a no-op if the dir already exists
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    // plain text, two lines: unix epoch first, then the task name.
    // epoch goes first so the task name can contain spaces and still just be
    // "the rest of the file" -- keeps the waybar script to pure bash, no jq
    let payload = format!("{}\n{}\n", started_at.timestamp(), task_name);

    // none of this is fatal -- a broken bar shouldn't break tracking
    if std::fs::write(&path, payload).is_ok() {
        signal_waybar();
    }
}

pub fn clear_active() {
    let Some(path) = state_path() else {
        return;
    };

    // remove_file errors if the file is already gone, which is fine
    let _ = std::fs::remove_file(&path);
    signal_waybar();
}
