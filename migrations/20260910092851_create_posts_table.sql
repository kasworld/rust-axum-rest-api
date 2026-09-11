-- migrations/<timestamp>_create_posts_table.sql
 
-- CREATE TABLE posts (
--     id SERIAL PRIMARY KEY,
--     user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
--     title TEXT NOT NULL,
--     body TEXT NOT NULL,
--     created_at TIMESTAMP DEFAULT NOW()
-- );

CREATE TABLE posts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER,
    title TEXT NOT NULL,
    body TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);