-- Your SQL goes here

CREATE TABLE chatrooms (
  chatroom_id SERIAL PRIMARY KEY,
  chatroom_name VARCHAR(20) NOT NULL
);

CREATE TABLE messages (
  id SERIAL PRIMARY KEY,
  creation TIMESTAMP NOT NULL,
  body VARCHAR(500) NOT NULL
)
