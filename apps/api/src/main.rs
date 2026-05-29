use axum::{extract::State, routing::get, Router};
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::net::SocketAddr;

#[derive(Clone)]
struct AppState {
    db: Pool<Sqlite>,
}

#[tokio::main]
async fn main() {
    let db = SqlitePoolOptions::new()
        .connect_lazy("sqlite://markdaily.db")
        .expect("failed to initialize sqlite pool");

    let app = Router::new()
        .route("/health", get(health))
        .route("/api/auth", get(auth_placeholder))
        .route("/api/docs", get(docs_placeholder))
        .route("/api/ai", get(ai_placeholder))
        .with_state(AppState { db });

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind server address");

    axum::serve(listener, app)
        .await
        .expect("failed to start axum server");
}

async fn health(State(state): State<AppState>) -> &'static str {
    let _ = state.db.is_closed();
    "ok"
}

async fn auth_placeholder() -> &'static str {
    "auth endpoint placeholder"
}

async fn docs_placeholder() -> &'static str {
    "markdown docs endpoint placeholder"
}

async fn ai_placeholder() -> &'static str {
    "ai query endpoint placeholder"
}
