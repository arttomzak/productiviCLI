mod cli;
mod config;
mod db;
mod tracker;

use crate::cli::commands::Commands;
use clap::Parser;

#[tokio::main]
async fn main() {
    
    let database_url = config::load(); // note that variables in rust are immutable by default!
    let args = cli::Cli::parse();

    // rust has object ownership, IF I PASS AN OBJECT INTO A FUNCTION AS IS, you move ownership into the func
    // now if you use an & you don't take the ownership, just look at it kinda like a C style amp
    // & references an object
    let pool = db::client::connect(&database_url).await;

    println!("we connected yo");
    // this is our super switch statement, which is exhaustive -> we gotta consider all commands

    // match takes a value and checks which pattern it fits in after Cli::parse()
    // takes what we typed and pulls out a command as defined within cli commands
    match args.command { 
        Commands::Start { task } => {
            // start TASKNAME
            tracker::session::start_session(&pool, &task).await;
            println!("Tracking your {} session", task)
        }

        Commands::Stop => {
            // stop
            tracker::session::stop_session(&pool).await;
            println!("Stopping session!");
        }

        // Commands::Status => {
        //     // status
        //     tracker::session:status_session(&pool).await;
        //     println!("status");
        // }

    }
}

