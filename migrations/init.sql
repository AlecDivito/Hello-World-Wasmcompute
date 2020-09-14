
CREATE TABLE users(
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    name        TEXT NOT NULL,
    description TEXT
);

CREATE TABLE comments(
    id      INTEGER PRIMARY KEY AUTOINCREMENT,
    comment TEXT NOT NULL, 
    user_id INTEGER NOT NULL,
    FOREIGN KEY(user_id) REFERENCES users(id)
);

INSERT INTO users(name, description) VALUES ('Alec', '2st Son');
INSERT INTO users(name, description) VALUES ('Deanna', '1st Daughter');
INSERT INTO users(name, description) VALUES ('Erik', '1st Son');
