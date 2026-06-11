// Module 06: Async Rust + Axum
//
// This is a complete working mini HTTP API. Read the code, run it, then complete
// the exercises in exercises.rs.
//
// Run:  cargo run -p module_06_async_axum
// Test: curl http://localhost:3000/health
//       curl http://localhost:3000/users
//       curl -X POST http://localhost:3000/users \
//         -H "Content-Type: application/json" \
//         -d '{"name":"Dylan","email":"dylan@example.com"}'

mod exercises;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

// ---- Data types ----

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

// ---- Shared state ----

#[derive(Clone)]
pub struct AppState {
    pub users: Arc<Mutex<Vec<User>>>,
    pub next_id: Arc<Mutex<u32>>,
}

impl AppState {
    fn new() -> Self {
        AppState {
            users: Arc::new(Mutex::new(vec![
                User { id: 1, name: "Alice".into(), email: "alice@example.com".into() },
                User { id: 2, name: "Bob".into(), email: "bob@example.com".into() },
            ])),
            next_id: Arc::new(Mutex::new(3)),
        }
    }
}

// ---- Handlers ----

async fn health_check() -> &'static str {
    "ok"
}

async fn list_users(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> Json<Vec<User>> {
    let users = state.users.lock().await;
    let offset = params.offset.unwrap_or(0);
    let limit = params.limit.unwrap_or(users.len());
    let page: Vec<User> = users.iter().skip(offset).take(limit).cloned().collect();
    Json(page)
}

async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> impl IntoResponse {
    let users = state.users.lock().await;
    match users.iter().find(|u| u.id == id) {
        Some(user) => Ok(Json(user.clone())),
        None => Err((StatusCode::NOT_FOUND, format!("User {} not found", id))),
    }
}

async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> (StatusCode, Json<User>) {
    let mut id_counter = state.next_id.lock().await;
    let id = *id_counter;
    *id_counter += 1;

    let user = User {
        id,
        name: payload.name,
        email: payload.email,
    };

    state.users.lock().await.push(user.clone());

    (StatusCode::CREATED, Json(user))
}

async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> impl IntoResponse {
    let mut users = state.users.lock().await;
    let before = users.len();
    users.retain(|u| u.id != id);

    if users.len() < before {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

// ---- Router ----

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/users", get(list_users).post(create_user))
        .route("/users/:id", get(get_user).delete(delete_user))
        .with_state(state)
}

#[tokio::main]
async fn main() {
    let state = AppState::new();
    let app = build_router(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server listening on http://localhost:3000");
    println!("Try:");
    println!("  curl http://localhost:3000/health");
    println!("  curl http://localhost:3000/users");
    println!("  curl http://localhost:3000/users/1");
    println!("  curl -X POST http://localhost:3000/users -H 'Content-Type: application/json' -d '{{\"name\":\"Dylan\",\"email\":\"dylan@example.com\"}}'");
    println!("  curl -X DELETE http://localhost:3000/users/1");

    axum::serve(listener, app).await.unwrap();
}
