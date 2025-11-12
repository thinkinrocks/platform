CREATE TABLE user (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    display_name TEXT NOT NULL,
    github TEXT,
    sire_id INTEGER,
    initiated_at TEXT NOT NULL
);

CREATE TABLE token (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    created_at TEXT NOT NULL,
    token TEXT NOT NULL,
    owner INTEGER NOT NULL,
    FOREIGN KEY(owner) REFERENCES user(id)
);

CREATE TABLE commitment (
    performed_at TEXT NOT NULL,
    token_id INTEGER NOT NULL,
    chlide_id INTEGER NOT NULL,
    FOREIGN KEY(token_id) REFERENCES token(id),
    FOREIGN KEY(chlide_id) REFERENCES user(id)
);
