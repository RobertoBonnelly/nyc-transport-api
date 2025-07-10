-- Add migration script here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE
);

CREATE TABLE county (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    coordinates geometry(POLYGON, 4326) NOT NULL
);

CREATE TABLE transport (
    id SERIAL PRIMARY KEY,
    type TEXT NOT NULL,
    destinations geometry(POLYGON, 4326) NOT NULL
)