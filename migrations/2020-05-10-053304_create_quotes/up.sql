-- Your SQL goes here
CREATE TABLE quotes (
       id SERIAL PRIMARY KEY,
       created_at TIMESTAMPTZ NOT NULL,
       price DECIMAL NOT NULL
)
