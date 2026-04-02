# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Run

```bash
# Build
cargo build

# Launch the TUI (default, no subcommand)
cargo run

# One-shot CLI commands (still work)
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

### sqlx Offline Mode

The project uses `sqlx` offline mode so CI can compile without a live database. Query metadata is cached in `.sqlx/` and committed to git.

After adding or modifying any `sqlx::query!` calls, run:

```bash
cargo sqlx prepare
```

Then commit the updated `.sqlx/` folder.

## Project Vision

A TUI productivity tracker that runs in a dedicated terminal pane and tracks work sessions on named tasks.

**Current state:**
- TUI launches by default (`cargo run`) with three panels: status, daily summary, command input
- `start <task>` and `stop` commands typed directly into the TUI
- Live session timer ticking every 250ms
- Daily summary showing total time per task today
- One-shot CLI commands (`start`, `stop`) still work for scripting

**Deployment direction:**
- Database is already hosted on Neon (cloud Postgres), accessible from any machine
- Next step: self-host Postgres on a VPS (e.g. Hetzner) when time allows — Neon is a stepping stone
- Eventually add a REST API backend and a React web dashboard for productivity insights

## Architecture

The app is a single async binary (`tokio::main`) that:
1. Loads `DATABASE_URL` from `.env` via `config::load()`
2. Parses CLI args with `clap` into an `Option<Commands>` enum
3. Opens a `sqlx::PgPool` connection
4. If a subcommand was given, runs it and exits. If no subcommand, launches the TUI.

**Module layout:**
- `cli/commands.rs` — defines the `Cli` struct and `Option<Commands>` enum
- `config.rs` — loads `.env` and returns `DATABASE_URL`
- `db/client.rs` — creates the `PgPool`
- `tracker/session.rs` — core logic: `start_session`, `stop_session`, `get_active_session`, `get_daily_summary`
- `tracker/task.rs` — `Task` struct with `sqlx::FromRow`
- `tui/mod.rs` — TUI entry point, event loop, layout, input handling

**Key data flow for `start`:**
- Look up task by name → create if missing → insert a session row with `started_at = NOW()`, `ended_at = NULL`

**Key data flow for `stop`:**
- Find the session where `ended_at IS NULL` → update with `ended_at = NOW()` and `duration_secs` via `EXTRACT(EPOCH FROM NOW() - started_at)`

**Key data flow for TUI:**
- On launch: query active session + daily summary
- Every 250ms: redraw (timer updates from `chrono::Utc::now() - started_at`)
- On Enter: parse command → call session function → refresh active session + daily summary

## Schema

```
tasks(id, name UNIQUE, created_at)
sessions(id, task_id → tasks.id, started_at, ended_at nullable, duration_secs nullable)
```

Active session = row in `sessions` where `ended_at IS NULL`.

## CI

GitHub Actions runs on every push to main:
- `cargo fmt --check` — formatting check
- `cargo build` — compile check (uses `SQLX_OFFLINE=true`)
- `cargo clippy` — lint check

Workflow file: `.github/workflows/ci.yml`

## Working Style

The user is actively learning Rust. Take things slow, explain concepts before writing code, and let them ask questions at each step. Key learnings are tracked in `NOTES.md`.
