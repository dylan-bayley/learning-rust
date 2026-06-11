// Module 06 Exercises — Async Rust + Axum
//
// Your goal: extend the API in main.rs with new functionality.
//
// Unlike previous modules, there are no unit tests here — you verify by
// running the server and curling the endpoints.
//
// Run:  cargo run -p module_06_async_axum
//
// ---- Exercise 1: Add a search endpoint ----------------------------------------
//
// Add a GET /users/search?name=<query> endpoint that returns all users whose
// name contains the query string (case-insensitive).
//
// Example:
//   curl "http://localhost:3000/users/search?name=ali"
//   → [{"id":1,"name":"Alice","email":"alice@example.com"}]
//
// Steps:
//   1. Add a SearchParams struct with a `name: String` field (use Deserialize)
//   2. Write a `search_users` handler that takes State and Query<SearchParams>
//   3. Register the route in build_router() in main.rs: GET /users/search
//
// ---- Exercise 2: Update a user ------------------------------------------------
//
// Add a PUT /users/:id endpoint that updates a user's name and/or email.
//
// Request body: { "name": "New Name", "email": "new@example.com" }
// Response: 200 with the updated User, or 404 if not found.
//
// Steps:
//   1. Add an UpdateUserRequest struct (both fields optional with Option<String>)
//   2. Write an `update_user` handler
//   3. Register the route: PUT /users/:id
//
// ---- Exercise 3: Pagination verification -------------------------------------
//
// The list_users endpoint already supports ?limit=N&offset=N but it's not tested.
// Verify it works:
//
//   curl "http://localhost:3000/users?limit=1"
//   → [{"id":1,...}]
//
//   curl "http://localhost:3000/users?limit=1&offset=1"
//   → [{"id":2,...}]
//
// ---- Exercise 4: Async basics -------------------------------------------------
//
// Below is a stub for an async function that simulates a slow database call.
// Implement it using tokio::time::sleep.

use tokio::time::{sleep, Duration};

/// Simulate fetching a user from a slow database.
/// Wait for `delay_ms` milliseconds, then return a User with the given id.
pub async fn slow_fetch_user(id: u32, delay_ms: u64) -> crate::User {
    todo!()
}

/// Fetch two users concurrently (not sequentially) and return both.
/// Use tokio::join! — look at the README section on concurrent futures.
pub async fn fetch_two_users_concurrently(id1: u32, id2: u32) -> (crate::User, crate::User) {
    todo!()
}

// ---- Tests for async exercises -----------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[tokio::test]
    async fn test_slow_fetch_user() {
        let user = slow_fetch_user(42, 10).await;
        assert_eq!(user.id, 42);
    }

    #[tokio::test]
    async fn test_concurrent_is_faster_than_sequential() {
        // Two 100ms fetches run concurrently should take ~100ms, not ~200ms
        let start = Instant::now();
        let (_u1, _u2) = fetch_two_users_concurrently(1, 2).await;
        let elapsed = start.elapsed().as_millis();

        // Should be well under 180ms (sequential would be ~200ms)
        assert!(
            elapsed < 180,
            "Expected concurrent fetch < 180ms, got {}ms — are you using join!?",
            elapsed
        );
    }
}
