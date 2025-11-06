# Stephen Coin — Coding Style Guide

This guide captures the specific conventions and less-common patterns used in this codebase (Rust + LuaJIT + SQLite). It omits general/common best-practices and focuses on the distinctive choices and idioms you should follow.

## Project layout & naming
- One module per file: `mod foo;` in `src/main.rs` corresponds to `src/foo.rs`.
- Files and Rust identifiers use snake_case (modules, functions, filenames).
- Keep non-Rust assets in project root (e.g., `schema.sql`, `README.md`).

## Asynchronous runtime and entry point
- Use Tokio as the async runtime and annotate the entry point:
  ```rust
  #[tokio::main]
  async fn main() -> std::io::Result<()> { ... }
  ```
- Prefer `async fn` for top-level operations (bootstrap, external integrations) and return Result-based types so callers can propagate errors.

## Result aliasing for clarity
- Alias crate-specific Result types at the top of files to make intent explicit:
  ```rust
  use rusqlite::{Connection, Result as SQLResult};
  use mlua::{Lua, Result as LuaResult};
  ```
  - Use `SQLResult` for database-returning functions and `LuaResult` for Lua interop functions.

## Lua (mlua) integration patterns
- Use `mlua` with async + LuaJIT enabled in Cargo features.
- Register Rust async functions into Lua via `create_async_function`:
  - Rust async function signature: include a `Lua` parameter and return a `LuaResult<T>`:
    ```rust
    async fn rust_fn(lua: Lua, arg: u64) -> LuaResult<&'static str> { ... }
    ```
  - Register and expose to Lua globals:
    ```rust
    lua.globals().set("sleep", lua.create_async_function(rust_fn)?)?;
    ```
- Execute Lua code with the async API:
  ```rust
  let res: String = lua.load(code).call_async(100).await?;
  ```
  - Always annotate the expected Rust type when calling Lua (e.g., `let res: String = ...`) to make conversions explicit.
- Use `call_async` to pass parameters to Lua chunks and `create_async_function` for Rust callbacks that await (e.g., `tokio::time::sleep`).

## Bridging async Rust and Lua
- Use Tokio timers inside Rust functions bound to Lua (e.g., `tokio::time::sleep(Duration::from_millis(n)).await;`).
- Export short, deterministic return values from Rust-to-Lua async functions (strings/integers) to keep conversions simple and safe.

## Database schema & migrations
- Keep the SQL schema in a root `schema.sql` file and load it at bootstrap:
  ```rust
  let schema = std::fs::read_to_string("schema.sql")?;
  conn.execute_batch(&schema)?;
  ```
  - Use `execute_batch` for multi-statement schema SQL (the codebase currently reads the file and intends to execute the full batch).
- Schema conventions (as used in `schema.sql`):
  - Explicit column types with length hints for addresses and signatures (e.g., `VARCHAR(64)`, `VARCHAR(7856)`).
  - Include columns for smart contracts:
    - `data BLOB` for arbitrary binary contract data.
    - `code TEXT` for Lua source.
  - Use inline `-- TODO` comments in SQL to record design notes or planned changes (e.g., Merkle root, primary keys).

## Cargo / dependency conventions
- Group dependencies by intent with comments in `Cargo.toml` (e.g., Quantum, Async, Ledger, Lua).
- Enable explicit feature flags for crates where behavior matters:
  - `mlua = { features = ["async", "luajit", "serde"] }`
  - `tokio = { features = ["full"] }`
  - `rusqlite = { features = ["bundled"] }`
  - Cryptography crates with targeted features (e.g., `pqc_sphincsplus` features).
- Use modern edition (project uses `edition = "2024"`).

## Error handling & logging style
- Prefer propagating errors via `-> Result<T, E>` and `?`.
- At integration points (bootstrap, external calls), match results to log outcomes instead of panicking:
  ```rust
  match conn.execute_batch(&schema) {
      Ok(_) => println!("Database schema loaded successfully."),
      Err(e) => println!("Error loading database schema: {}", e),
  };
  ```
- Temporary `unwrap()` usage is acceptable with TODOs where full error handling is deferred — mark as `// TODO` when present.

## Assertions and runtime checks
- Use `assert_eq!` for internal sanity checks after calling into Lua or other deterministic operations, to fail fast during development:
  ```rust
  assert_eq!(res, "done");
  ```
- Keep these assertions for critical invariants; comment or remove before production if they could be triggered by user input.

## SQL & contract field patterns
- Smart contract source stored as TEXT in the same transactions table (`code TEXT`), and payload as `BLOB`.
- Include `note VARCHAR(256)` for human-readable metadata.
- Keep `timestamp` as INTEGER (epoch ms/seconds depending on convention used elsewhere).

## TODOs and incremental development notes
- Use `TODO` comments liberally in SQL and Rust for planned work (schema changes, unwrap fixes, algorithm design).
- Track TODOs in README for high-level roadmap items (consensus, difficulty algorithm, LuaJIT/dockerization).

## Git and artifacts
- Ignore build artifacts and database files in `.gitignore`:
  - `/target`
  - `*.db`

---

Follow these conventions when adding new modules, database fields, Lua bindings, or dependency features to keep the codebase coherent and compatible with the current async + LuaJIT + SQLite integration patterns.