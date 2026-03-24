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

The app requires a running PostgreSQL instance via Docker. From the project root:

```bash
docker-compose up -d
```

Connection is configured via `DATABASE_URL` in a `.env` file (see `.env.example`).

Migrations must be applied manually — there is no migration runner yet. Run each file in order:

```bash
docker exec -i productivicli-postgres-1 psql -U productivicli -d productividb < migrations/001_create_tasks.sql
docker exec -i productivicli-postgres-1 psql -U productivicli -d productividb < migrations/002_create_sessions.sql
```

`sqlx` uses compile-time query checking, so `DATABASE_URL` must be set and the schema must exist before the project will compile.

## Project Vision

A CLI productivity tracker where the user can run commands at any time to track work sessions on named tasks. Long-term goal is to move the database to a hosted/deployed instance and build a web dashboard that pulls session data to visualize productivity insights.

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
