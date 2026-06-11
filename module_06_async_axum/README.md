# Module 06: Async Rust and Axum

You've learned the core language. Now we put it to work building an HTTP API — the actual job.

---

## 1. async / await

Rust's async model is similar to Python's asyncio but has key differences.

```python
# Python asyncio
import asyncio

async def fetch_data(url: str) -> str:
    await asyncio.sleep(1)
    return "data"

async def main():
    result = await fetch_data("http://example.com")
    print(result)

asyncio.run(main())
```

```rust
// Rust + tokio
use tokio::time::{sleep, Duration};

async fn fetch_data(url: &str) -> String {
    sleep(Duration::from_secs(1)).await;
    "data".to_string()
}

#[tokio::main]
async fn main() {
    let result = fetch_data("http://example.com").await;
    println!("{}", result);
}
```

Key differences from Python asyncio:
- Rust async functions return a **Future** (like Python's coroutine) — nothing runs until `.await`ed
- You need a **runtime** to execute futures. We use `tokio` (the standard choice)
- `#[tokio::main]` is a macro that sets up the tokio runtime for your main function

### Running multiple things concurrently

```rust
use tokio::join;

// Run two futures concurrently (like asyncio.gather)
let (result1, result2) = join!(fetch_data("url1"), fetch_data("url2")).await;

// Or with tokio::spawn for independent tasks (like asyncio.create_task)
let handle = tokio::spawn(async {
    fetch_data("url").await
});
let result = handle.await.unwrap();
```

---

## 2. Axum Basics

Axum is the most ergonomic Rust web framework. It's built on `tower` (middleware) and `tokio` (async runtime).

### Hello World

```rust
use axum::{routing::get, Router};

async fn hello() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on :3000");
    axum::serve(listener, app).await.unwrap();
}
```

### Path and Query Parameters

```rust
use axum::{extract::{Path, Query}, routing::get, Router};
use serde::Deserialize;

// Path parameter: /users/42
async fn get_user(Path(user_id): Path<u32>) -> String {
    format!("User {}", user_id)
}

// Query parameters: /search?q=rust&limit=10
#[derive(Deserialize)]
struct SearchParams {
    q: String,
    limit: Option<u32>,
}

async fn search(Query(params): Query<SearchParams>) -> String {
    format!("Searching for '{}', limit: {:?}", params.q, params.limit)
}
```

### JSON Request and Response

```rust
use axum::{extract::Json, routing::post, Router};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

#[derive(Serialize)]
struct User {
    id: u32,
    name: String,
    email: String,
}

async fn create_user(Json(payload): Json<CreateUser>) -> Json<User> {
    Json(User {
        id: 1,
        name: payload.name,
        email: payload.email,
    })
}
```

### Returning Errors from Handlers

Axum handlers can return `Result` if the error type implements `IntoResponse`:

```rust
use axum::{http::StatusCode, response::IntoResponse};

async fn get_user(Path(id): Path<u32>) -> Result<Json<User>, (StatusCode, String)> {
    if id == 0 {
        return Err((StatusCode::NOT_FOUND, "User not found".to_string()));
    }
    Ok(Json(User { id, name: "Dylan".into(), email: "d@example.com".into() }))
}
```

### Shared State

```rust
use axum::extract::State;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
struct AppState {
    db: Arc<Mutex<Vec<User>>>,
}

async fn list_users(State(state): State<AppState>) -> Json<Vec<User>> {
    let db = state.db.lock().await;
    Json(db.clone())
}

#[tokio::main]
async fn main() {
    let state = AppState {
        db: Arc::new(Mutex::new(vec![])),
    };

    let app = Router::new()
        .route("/users", get(list_users))
        .with_state(state);
    // ...
}
```

---

## 3. Serde

`serde` is Rust's serialization/deserialization library. With `serde_json` you can convert structs to/from JSON.

```rust
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    host: String,
    port: u16,
    debug: bool,
}

// Struct → JSON string
let config = Config { host: "localhost".into(), port: 8080, debug: true };
let json = serde_json::to_string(&config).unwrap();
// {"host":"localhost","port":8080,"debug":true}

// JSON string → Struct
let config2: Config = serde_json::from_str(&json).unwrap();
```

---

## Exercise: Build a Mini Users API

The exercise for this module is a complete mini HTTP API. See `src/main.rs` for the starting structure and `src/exercises.rs` for what you need to implement.

```bash
# Run the server
cargo run -p module_06_async_axum

# Test it (in another terminal)
curl http://localhost:3000/health
curl http://localhost:3000/users
curl -X POST http://localhost:3000/users \
  -H "Content-Type: application/json" \
  -d '{"name":"Dylan","email":"dylan@example.com"}'
```

For the API exercise there are no unit tests — run the server and use curl or a tool like [Insomnia](https://insomnia.rest/) to verify it works.
