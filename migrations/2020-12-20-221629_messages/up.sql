-- Your SQL goes here

CREATE TABLE chatroom (
  chatroom_id SERIAL PRIMARY KEY,
  chatroom_name VARCHAR(20) NOT NULL
);

CREATE TABLE message (
  id SERIAL PRIMARY KEY,
  body VARCHAR(500) NOT NULL,
  chatroom_id INTEGER NOT NULL
)