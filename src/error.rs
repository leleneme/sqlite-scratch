// Copyright (C) 2025  leleneme
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use serde::Serialize;
use std::fmt;

#[derive(Serialize)]
pub struct ApiError {
    error: String,
}

impl ApiError {
    pub fn from_str(message: &str) -> Self {
        Self {
            error: message.to_string(),
        }
    }

    pub fn from(message: String) -> Self {
        Self { error: message }
    }
}

pub enum ResolveError {
    DoesNotExists,
    NotProvided,
}

impl fmt::Display for ResolveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let desc = match self {
            Self::NotProvided => {
                "Path to the SQLite database must be provided either as first argument or as DB_FILE environment variable"
            }
            Self::DoesNotExists => "Provided database file does not exists",
        };

        f.write_str(desc)
    }
}
