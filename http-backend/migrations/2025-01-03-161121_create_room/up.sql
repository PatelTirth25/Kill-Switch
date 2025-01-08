-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL,
    password VARCHAR NOT NULL
);

CREATE TABLE rooms (
    id SERIAL PRIMARY KEY,
    room VARCHAR NOT NULL,
    url VARCHAR NOT NULL
);
