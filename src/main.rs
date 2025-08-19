// Copyright (C) 2025  leleneme
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

mod assets;
mod database;
mod error;

use assets::static_handler;
use database::{get_schema, run_query};
use error::{ApiError, ResolveError};

use axum::{
    Json, Router,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use rusqlite::Connection;
use serde::Deserialize;

use std::path::PathBuf;
use std::sync::{Arc, Mutex};

#[derive(Deserialize)]
struct QueryParams {
    q: Option<String>,
}

#[derive(Clone)]
struct AppState {
    conn: Arc<Mutex<Connection>>,
}

async fn schema_handle(State(state): State<AppState>) -> impl IntoResponse {
    let conn = state.conn.lock().unwrap();
    match get_schema(&conn) {
        Ok(schema) => Json(schema).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiError::from(e.to_string())),
        )
            .into_response(),
    }
}

async fn query_handle(
    State(state): State<AppState>,
    Query(params): Query<QueryParams>,
) -> impl IntoResponse {
    let conn = state.conn.lock().unwrap();
    match params.q {
        Some(sql) => match run_query(&conn, &sql) {
            Ok(result) => Json(result).into_response(),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiError::from(e.to_string())),
            )
                .into_response(),
        },
        Option::None => (
            StatusCode::BAD_REQUEST,
            Json(ApiError::from_str("Missing 'q' parameter")),
        )
            .into_response(),
    }
}

fn resolve_db_path() -> Result<PathBuf, ResolveError> {
    use std::env::{args, var};

    fn check_path<P: Into<PathBuf>>(p: P) -> Result<PathBuf, ResolveError> {
        let path = p.into();
        if path.exists() {
            Ok(path)
        } else {
            Err(ResolveError::DoesNotExists)
        }
    }

    if let Some(arg) = args().nth(1) {
        check_path(arg)
    } else if let Ok(env_path) = var("DB_FILE") {
        check_path(env_path)
    } else {
        Err(ResolveError::NotProvided)
    }
}

#[tokio::main]
async fn main() {
    let db_path = match resolve_db_path() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1)
        }
    };

    let conn = Connection::open(&db_path).expect("Failed to open SQLite database");
    let state = AppState {
        conn: Arc::new(Mutex::new(conn)),
    };

    let app = Router::new()
        .route("/api/schema", get(schema_handle))
        .route("/api/query", get(query_handle))
        .fallback(static_handler)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3030")
        .await
        .unwrap();

    println!("Listening at http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
