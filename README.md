rustdb-api-async ⚡

An async HTTP key-value API built with Tokio and Axum, backed by a persistent append-only log.

This is the async rewrite of rustdb-api, which implemented HTTP by hand over raw TcpListener and threads. Building the manual version first made it clear what Axum and Tokio are actually doing for you.

Demo
curl -i -X POST http://127.0.0.1:8080/set -d "city=nairobi"
# HTTP/1.1 200 OK        → OK

curl http://127.0.0.1:8080/get/city
# nairobi

curl -i -X DELETE http://127.0.0.1:8080/delete/city
# HTTP/1.1 204 No Content

curl -i http://127.0.0.1:8080/get/city
# HTTP/1.1 404 Not Found

Restart the server and the data (and the deletes) are still reflected, because state is rebuilt from db.log on startup.

Features
Async server — Tokio runtime, Axum routing and extractors

Shared state — Arc<Mutex<HashMap>> injected into handlers via State

Persistence — every write is appended to db.log and replayed on startup

Input validation — keys containing whitespace or empty keys are rejected with 400
Correct status codes — 200, 204, 400, 404

Getting Started
Prerequisites
Rust (1.75 or later)

git clone https://github.com/yourusername/rustdb-api-async
cd rustdb-api-async
cargo run

[dependencies]
tokio = { version = "1", features = ["full"] }
axum = "0.8"


Axum 0.8 uses /{param} for path captures. Older tutorials use /:param, which panics at startup on 0.8.

Resources
Tokio tutorial (https://tokio.rs/tokio/tutorial)
Axum documentation (https://docs.rs/axum/0.8)
Axum examples (https://github.com/tokio-rs/axum/tree/main/examples)
Bitcask paper (https://riak.com/assets/bitcask-intro.pdf)
