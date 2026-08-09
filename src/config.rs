// Loads environment variables from .env
// Will hold database URL and any other config values

use std::path::PathBuf;

// ~/.config/productivicli/env
// When the app is launched from rofi instead of from a terminal, the working
// directory is $HOME -- so the project's own .env is nowhere to be found.
// This is the copy that installed runs read from.
fn config_env_path() -> Option<PathBuf> {
    // XDG_CONFIG_HOME if it's set, otherwise fall back to ~/.config
    let base = match std::env::var("XDG_CONFIG_HOME") {
        Ok(dir) => PathBuf::from(dir),
        Err(_) => PathBuf::from(std::env::var("HOME").ok()?).join(".config"),
    };

    Some(base.join("productivicli").join("env"))
}

pub fn load() -> String {
    // dotenvy never overwrites a variable that is already set, so the order
    // here is a precedence list: real env wins, then the project .env, then
    // the installed config file.
    dotenvy::dotenv().ok(); // .ok() keeps us from crashing on an empty env call

    if let Some(path) = config_env_path() {
        dotenvy::from_path(&path).ok();
    }

    // .expect(xyz) at the end will give a val or crash w xyz message
    std::env::var("DATABASE_URL").expect(
        "DATABASE_URL is not set yo -- put it in ./.env for dev, \
         or ~/.config/productivicli/env for installed runs",
    )

    // cool rust thing, if you don't have a semicolon on the last line
    // itll auto return
}
