## productiviCLI

productiviCLI is a simple CLI tool that lets you track your time spent 
on anything!

This tool currently writes session data to a postgreSQL database that I 
personally have deployed through Neon, and runs in a TUI that showcases
the task that you're currently tracking, and a daily/weekly summary.

Down the line I want to create a little waybar module that will display a
the time spent on your current task and click into the tui

# Installation

Clone the repository

Run the migrations within migrations/ and replicate the schema within your postgreSQL database.

Create an .env file and add your database url 

run cargo run and you're good to go!

# Docs

- [docs/rofi-launcher.md](docs/rofi-launcher.md) — make the app searchable in rofi and open in its own terminal window
- [docs/NOTES.md](docs/NOTES.md) — Rust learning notes


