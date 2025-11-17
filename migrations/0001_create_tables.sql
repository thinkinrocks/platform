CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    display_name TEXT NOT NULL,
    access_code_hash CHAR(64) NOT NULL CHECK (length(access_code_hash) = 64),
    sign_up_code_id INTEGER NULL,  -- allow NULL for optional foreign key
    initiated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS sign_up_codes (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    granted_by INTEGER NOT NULL,
    code TEXT NOT NULL,
    at TEXT NOT NULL,
    FOREIGN KEY (granted_by) REFERENCES users(id) ON DELETE CASCADE
);

PRAGMA foreign_keys = ON;
CREATE INDEX IF NOT EXISTS idx_users_access_code_hash ON users(access_code_hash);
CREATE INDEX IF NOT EXISTS idx_users_sign_up_code_id ON users(sign_up_code_id);
CREATE INDEX IF NOT EXISTS idx_sign_up_codes_granted_by ON sign_up_codes(granted_by);
