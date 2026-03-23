// Defines all CLI subcommands
// e.g. start, stop, status, log, report

// self note #[] is basically a decorator -> attaches attributes to things in Rust
// a tag that you stick onto an object

// imports Parser and Subcommand traits
use clap::{Parser, Subcommand};


#[derive(Parser)]
#[command(name = "productivicli", about = "Track your productivity in a CLI (your productiviCLI :) )")]
pub struct Cli { // container that holds what command the user types
    #[command(subcommand)] // tells us yo command isn't a flag, but its a subcommand that is start or stop
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Start { task: String },
    Stop,
}