default:
    just --list

startup-db:
    sqlite3 app.sqlite3 < migrations/0001_create_tables.sql
