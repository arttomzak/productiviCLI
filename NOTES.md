# Rust Notes

Personal reference based on things that came up while building productiviCLI.

---

## Variables

Variables are immutable by default. Use `mut` to make them changeable.

```rust
let x = 5;        // immutable
let mut y = 5;    // mutable
```

Rust infers the type automatically — you don't need to declare it explicitly most of the time.

---

## Functions

The last expression in a function without a semicolon is automatically returned.

```rust
pub fn load() -> String {
    dotenvy::dotenv().ok();
    std::env::var("DATABASE_URL").expect("DATABASE_URL not set") // returned
}
```

This is the same as writing `return ...;` at the end. Rust encourages the no-semicolon style.

The `-> String` in the signature tells Rust what type the function returns.

---

## Semicolons

- `;` at the end of a line means "run this, throw away the result"
- No `;` on the last line means "this is the return value"

---

## References and Borrowing (`&`)

Every value in Rust has exactly one owner. When you pass a value to a function you move ownership — you can't use it after.

`&` means borrow — lend the value without giving up ownership.

```rust
let database_url = config::load(); // main() owns this
db::client::connect(&database_url) // connect() borrows it, main() still owns it
```

`&str` is a borrowed string slice — the most common way to pass strings around without transferring ownership.

Similar to a pointer in C but the compiler guarantees it is always valid. No dangling pointers.

---

## Option<T>

Rust has no `null`. Instead it uses `Option<T>` to represent a value that might not exist.

```rust
ended_at: Option<chrono::DateTime<chrono::Utc>>,  // null = None, has value = Some(...)
```

This forces you to handle the "missing" case explicitly instead of crashing at runtime.

---

## Result<T, E>

Rust functions that can fail return `Result<T, E>` — either `Ok(value)` or `Err(error)`.

```rust
std::env::var("DATABASE_URL")  // returns Result<String, VarError>
    .expect("DATABASE_URL not set")  // unwrap Ok, or crash with this message
```

`.expect()` is the quick way to handle it during development. In production you'd handle the error more gracefully.

---

## Structs

Rust's way of defining a data structure. Fields use `:` between name and type, separated by commas.

```rust
pub struct Task {
    id: i32,
    name: String,
    created_at: chrono::DateTime<chrono::Utc>,
}
```

---

## Traits

A trait defines behavior a type can have — like an interface. You implement traits for your types.

Some traits can be auto-implemented using `derive` — Rust generates the code for you at compile time.

---

## Derive Macros

`#[derive(...)]` must sit directly above the struct with no blank lines. It generates trait implementations automatically at compile time.

```rust
#[derive(Debug, sqlx::FromRow)]
pub struct Task { ... }
```

- `Debug` — lets you print the struct with `{:?}` for debugging
- `sqlx::FromRow` — lets sqlx map a database row directly to this struct

---

## Async / Await

Async code can pause and wait without blocking everything else. Like a waiter taking multiple orders instead of standing at the kitchen waiting for one.

`async fn` marks a function as async. `.await` pauses execution until the async operation completes.

```rust
let pool = db::client::connect(&database_url).await;
```

Requires an async runtime — this project uses `tokio`.

---

## `#[tokio::main]`

An attribute macro that turns `main()` into an async entry point. Without it, `main` can't be async and you can't use `.await` at the top level.

```rust
#[tokio::main]
async fn main() {
    // async code works here
}
```

---

## The Standard Library (`std`)

Built into every Rust installation. Organized into modules navigated with `::`.

```rust
std::env::var("DATABASE_URL")
// std → standard library
// env → environment module
// var() → function that reads an env variable
```

---

## Modules (`mod`)

`mod` tells Rust that other files exist and should be included in the project. Without it Rust won't know about your other files.

```rust
mod cli;
mod config;
mod db;
mod tracker;
```

`::` navigates into modules — `db::client::connect()` means the `connect` function inside `client.rs` inside the `db` module.

---

## Attributes (`#[...]`)

`#[...]` is how you attach metadata or instructions to structs, functions, or fields. The compiler or a library reads them at compile time and acts on them.

```rust
#[tokio::main]          // "run this as an async entry point"
#[derive(Debug)]        // "auto-implement the Debug trait"
#[command(subcommand)]  // "treat this field as a subcommand" (clap)
```

Think of it like a decorator in Python or an annotation in Java — it changes behavior without being part of the logic itself.

---

## Enums

A type that can be exactly one of several variants. Used with clap to define CLI subcommands.

```rust
pub enum Commands {
    Start { task: String },  // productivicli start "coding"
    Stop,                    // productivicli stop
}
```

Each variant can optionally hold data. `Start` holds a `task` string, `Stop` holds nothing.

---

## Traits vs Attribute Options — Capitalization Matters

`Subcommand` (capital S) is a **trait** you derive on an enum:
```rust
#[derive(Subcommand)]
pub enum Commands { ... }
```

`subcommand` (lowercase) is a **clap attribute option** that marks a field as holding a subcommand:
```rust
#[command(subcommand)]
pub command: Commands,
```

Same word, two different things. Capitalization tells them apart.

---

## `use`

Imports items from a crate or module so you can use them without the full path.

```rust
use clap::{Parser, Subcommand};
// now you can write Parser instead of clap::Parser
```

---

## Why `mod` exists

Rust does not automatically include every file in your project. You have to explicitly declare what exists with `mod`. Without it, Rust ignores the file entirely — no error, it just doesn't exist to the compiler.

```rust
mod cli;      // "src/cli/mod.rs exists, pull it in"
mod db;       // "src/db/mod.rs exists, pull it in"
mod config;   // "src/config.rs exists, pull it in"
```

`mod` is short for **module** — a named container for related code. Other languages call these packages, namespaces, or directories. Rust calls them modules.

The folder structure mirrors the module tree:
```
mod cli        → src/cli/mod.rs
mod db         → src/db/mod.rs
mod tracker    → src/tracker/mod.rs
mod config     → src/config.rs
```

Rust infers the file path from the module name — no explicit paths needed like in JavaScript.

## `pub mod` vs `pub use`

```rust
pub mod commands;      // "commands.rs exists, include it as a submodule"
pub use commands::Cli; // "re-export Cli so outside code can use it as cli::Cli"
```

`pub mod` makes a submodule visible. `pub use` re-exports something from inside that submodule so callers don't have to dig into it themselves.

---

## `crate::` — the project root

`crate::` means "start from the root of this project." Use it when Rust can't find something with a relative path.

```rust
use crate::cli::commands::Commands;
// start at root → go into cli → go into commands → bring in Commands
```

Without `crate::` Rust looks relative to the current file, which may not find what you need.

---

## `match`

Pattern matching — like a `switch` statement but exhaustive and more powerful. Rust forces you to handle every possible variant. If you miss one, your code won't compile.

```rust
match args.command {
    Commands::Start { task } => {
        // runs when user typed "start <task>"
        // task is automatically extracted and available here
    }
    Commands::Stop => {
        // runs when user typed "stop"
    }
}
```

The `{ task }` part is **destructuring** — pulling a field out of an enum variant directly in the match arm so you can use it as a plain variable inside the block.

---

## Imports (`use`)

Bring items into scope so you don't need to write the full path every time.

```rust
use clap::Parser;                      // bring in a single item
use clap::{Parser, Subcommand};        // bring in multiple items at once
use crate::cli::commands::Commands;    // bring in something from your own project
```

Items from traits (like `.parse()` from `Parser`) only work if the trait is in scope. If you get "method not found", check if you need a `use` statement for the trait.
