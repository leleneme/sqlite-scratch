// Copyright (C) 2025  leleneme
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use axum::{
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
};
use rust_embed::Embed;

#[derive(Embed)]
#[folder = "static/"]
struct Asset;

pub async fn static_handler(req: Request<axum::body::Body>) -> Response {
    let path = req.uri().path().trim_start_matches("/");
    let path = if path.is_empty() { "index.html" } else { &path };

    match Asset::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            Response::builder()
                .header("Content-Type", mime.as_ref())
                .body(content.data.into())
                .unwrap()
        }
        None => StatusCode::NOT_FOUND.into_response(),
    }
}
