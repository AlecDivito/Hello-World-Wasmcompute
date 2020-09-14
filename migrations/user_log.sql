
CREATE TABLE user_log(
    id       INTEGER PRIMARY KEY AUTOINCREMENT,
    duration INTEGER NOT NULL,
    addr     TEXT NOT NULL,
    method   TEXT NOT NULL,
    url_path TEXT NOT NULL,
    status   TEXT NOT NULL
);
