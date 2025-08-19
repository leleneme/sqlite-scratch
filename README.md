### sql-scratch

A simple and stupid web application for running queries and seeing the schema of a SQLite database.

#### Building

This project depends on `cargo`.

```
$ cargo build --release
$ cp target/release/sqlite-scratch .
```

#### Usage

```
sql-scratch [database file]
```

(If the database file is not provided as a command-line argument, DB_FILE environment variable is used as fallback.)

#### License

This project is licensed under the GNU GPL version 3 or any later version. See [LICENSE](./LICENSE) for details.