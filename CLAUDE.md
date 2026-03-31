# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Run

```bash
# Build
cargo build

# Run a command
cargo run -- start <task-name>
cargo run -- stop

# Check for compile errors without building a binary
cargo check
```

## Database Setup

The database is hosted on **Neon** (cloud Postgres). Docker is no longer used for the database.

Connection is configured via `DATABASE_URL` in a `.env` file. Use the **direct (non-pooled)** connection string from Neon — the pooled URL causes prepared statement errors during `sqlx` compile-time query checking.

Migrations must be applied manually. Run each file using `psql` with the direct connection URL:

```bash
psql "$DATABASE_URL" < migrations/001_create_tasks.sql
psql "$DATABASE_URL" < migrations/002_create_sessions.sql
```

Alternatively, run them directly from the Neon dashboard SQL editor.

`sqlx` uses compile-time query checking, so `DATABASE_URL` must be set and the schema must exist before the project will compile.

## Project Vision

A CLI productivity tracker where the user can run commands at any time to track work sessions on named tasks.

**Deployment direction:**
- Database is already hosted on Neon (cloud Postgres), accessible from any machine
- Next step: self-host Postgres on a VPS (e.g. Hetzner) when time allows — Neon is a stepping stone
- Eventually add a REST API backend and a React web dashboard for productivity insights

**Long-term UI direction:**
- Replace the plain CLI output with a **TUI** (Terminal User Interface) using `ratatui`
- Goal is a live terminal view with session timer, recent session table, and stats — always running in a terminal window rather than one-shot commands

## Architecture

The app is a single async binary (`tokio::main`) that:
1. Loads `DATABASE_URL` from `.env` via `config::load()`
2. Parses CLI args with `clap` into a `Commands` enum
3. Opens a `sqlx::PgPool` connection
4. Dispatches to `tracker::session` functions

**Module layout:**
- `cli/commands.rs` — defines the `Cli` struct and `Commands` enum
- `config.rs` — loads `.env` and returns `DATABASE_URL`
- `db/client.rs` — creates the `PgPool`
- `tracker/session.rs` — core logic: `start_session` and `stop_session`
- `tracker/task.rs` — `Task` struct with `sqlx::FromRow`

**Key data flow for `start`:**
- Look up task by name → create if missing → insert a session row with `started_at = NOW()`, `ended_at = NULL`

**Key data flow for `stop`:**
- Find the session where `ended_at IS NULL` → update with `ended_at = NOW()` and `duration_secs` via `EXTRACT(EPOCH FROM NOW() - started_at)`

## Schema

```
tasks(id, name UNIQUE, created_at)
sessions(id, task_id → tasks.id, started_at, ended_at nullable, duration_secs nullable)
```

Active session = row in `sessions` where `ended_at IS NULL`.

## Planned Commands

Beyond the current `start` and `stop`, the intended commands are:
- `status` — show if a session is currently active, which task, and for how long
- `log` — list recent sessions with task name, duration, and date
- `report` — total time per task over a time range

## Working Style

The user is actively learning Rust. Take things slow, explain concepts before writing code, and let them ask questions at each step. Key learnings are tracked in `NOTES.md`.
