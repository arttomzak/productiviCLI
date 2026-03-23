pub mod commands;

// Entry point for CLI parsing via clap
// Connects user input to tracker actions

// THE POINT OF THIS IS TO ACT AS AN EXPORTER TO THE REST OF OUR APP
// this module file will later let us call something like cli::Cli based on our mod Cli in main for example!

pub use commands::Cli;