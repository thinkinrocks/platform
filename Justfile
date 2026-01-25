run:
    cargo run

db:
    sqlite3 sqlite.db ".read src/setup.sql"
