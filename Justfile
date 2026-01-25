run:
    cargo run

db:
    sqlite3 sqlite.db ".read src/sql/setup.sql"

demo-db:
    rm -rf sqlite.db
    just db
    sqlite3 sqlite.db ".read samples/samples.sql"
