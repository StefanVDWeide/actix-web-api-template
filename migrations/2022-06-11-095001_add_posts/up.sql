-- Your SQL goes here
CREATE TABLE posts (
    id SERIAL NOT NULL PRIMARY KEY,
    title TEXT NOT NULL,
    body TEXT NOT NULL,
    user_id INT NOT NULL REFERENCES users (id),
    created_at TIMESTAMP NOT NULL
);
