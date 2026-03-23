mod cli;
mod config;
mod db;
mod tracker;

#[tokio::main]
async fn main() {
    let database_url = config::load(); // note that variables in rust are immutable by default!

    // rust has object ownership, IF I PASS AN OBJECT INTO A FUNCTION AS IS, you move ownership into the func
    // now if you use an & you don't take the ownership, just look at it kinda like a C style amp
    // & references an object
    let _pool = db::client::connect(&database_url).await;

    println!("we connected yo");
}
