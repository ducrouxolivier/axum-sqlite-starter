# axum-sqlite-starter

A small JSON API on [Axum](https://github.com/tokio-rs/axum) 0.8 and [SQLx](https://github.com/launchbadge/sqlx) 0.8 with SQLite.
Backend only. No external services: the database is a file created on first run, and migrations run on boot.

- `GET /up` health check, 200 only when the database answers
- `GET|POST /api/notes`, `GET|PUT|DELETE /api/notes/{id}` example CRUD resource
- JSON errors (`404`, `422`), request tracing, permissive CORS
- Integration tests against an in-memory database

## Requirements

Rust 1.85 or newer (`rustup`). A C compiler (`build-essential` / Xcode CLT), which the bundled SQLite needs.

## Setup

```sh
cargo build
```

No database setup step. `data/app.db` is created on first run.

## Run

```sh
cargo run
```

Listens on `0.0.0.0:3000`. Check it with `curl http://localhost:3000/up`.

| Variable | Default | |
|---|---|---|
| `HOST` | `0.0.0.0` | bind address |
| `PORT` | `3000` | bind port |
| `DATABASE_URL` | `sqlite://data/app.db?mode=rwc` | SQLite file |
| `RUST_LOG` | `info,tower_http=info` | log filter |

## Test

```sh
cargo test
```

## Layout

```
src/main.rs        boot: config, database, migrations, server
src/lib.rs         router and shared state
src/routes/        health and notes handlers
src/error.rs       AppError -> JSON response
migrations/        SQL migrations (sqlx), applied on boot
tests/api.rs       request-level tests
```

## License

MIT
