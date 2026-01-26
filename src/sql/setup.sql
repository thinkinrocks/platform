PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS users (
    telegram_username TEXT(32) NOT NULL PRIMARY KEY UNIQUE,
    sire TEXT(32),
    FOREIGN KEY (sire) REFERENCES users (telegram_username)
);

CREATE TABLE IF NOT EXISTS images (
    id TEXT(88) PRIMARY KEY UNIQUE,
    data BLOB
);

CREATE TABLE IF NOT EXISTS entries (
    id TEXT(21) PRIMARY KEY UNIQUE,
    name TEXT NOT NULL,
    image TEXT(88),
    description TEXT,
    note TEXT,
    created_at TEXT,
    stored_in TEXT(21),
    responsible_person TEXT(32),
    FOREIGN KEY (image) REFERENCES images (id) ON DELETE SET NULL,
    FOREIGN KEY (stored_in) REFERENCES entries (id) ON DELETE SET NULL,
    FOREIGN KEY (responsible_person) REFERENCES users (telegram_username) ON DELETE SET NULL
);

CREATE INDEX idx_entries_image ON entries (image);

CREATE INDEX idx_entries_stored_in ON entries (stored_in);

CREATE INDEX idx_entries_responsible_person ON entries (responsible_person);

CREATE INDEX idx_entries_name ON entries (id);