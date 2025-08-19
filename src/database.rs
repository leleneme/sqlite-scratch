// Copyright (C) 2025  leleneme
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use rusqlite::{Connection, Row};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct QueryResult {
    sql: String,
    columns: Vec<String>,
    rows: Vec<Vec<serde_json::Value>>,
}

#[derive(Serialize)]
struct Column {
    name: String,
    r#type: String,
    is_pk: bool,
    default: Option<String>,
    not_null: bool,
}

#[derive(Serialize)]
pub struct Schema {
    tables: HashMap<String, Vec<Column>>,
}

fn to_json(value: rusqlite::types::Value) -> serde_json::Value {
    use rusqlite::types::Value::*;

    match value {
        Null => serde_json::Value::Null,
        Integer(i) => serde_json::Value::from(i),
        Real(f) => serde_json::Value::from(f),
        Text(t) => serde_json::Value::from(t),
        Blob(_) => serde_json::Value::String("[BLOB]".to_string()),
    }
}

pub fn run_query(conn: &Connection, sql: &str) -> rusqlite::Result<QueryResult> {
    let mut stmt = conn.prepare(sql)?;
    let column_names: Vec<String> = stmt.column_names().iter().map(|s| s.to_string()).collect();

    let rows = stmt
        .query_map([], |row: &Row| {
            (0..column_names.len())
                .map(|i| row.get(i).map(to_json))
                .collect()
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;

    Ok(QueryResult {
        sql: sql.to_string(),
        columns: column_names,
        rows,
    })
}

pub fn get_schema(conn: &Connection) -> rusqlite::Result<Schema> {
    let mut tables = HashMap::new();
    let mut stmt = conn.prepare(
        r#"SELECT tbl_name FROM sqlite_master
           WHERE type='table'
           AND tbl_name NOT LIKE 'sqlite%';"#,
    )?;

    let table_names: Vec<String> = stmt
        .query_map([], |row| row.get(0))?
        .collect::<rusqlite::Result<Vec<_>>>()?;

    for table in table_names {
        let sql = format!("PRAGMA table_info({table})");
        let mut col_stmt = conn.prepare(&sql)?;

        let cols = col_stmt
            .query_map([], |row: &Row| {
                Ok(Column {
                    name: row.get("name")?,
                    r#type: row.get("type")?,
                    is_pk: row.get::<_, i32>("pk")? != 0,
                    default: row.get("dflt_value")?,
                    not_null: row.get::<_, i32>("notnull")? != 0,
                })
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;

        tables.insert(table, cols);
    }

    Ok(Schema { tables })
}
