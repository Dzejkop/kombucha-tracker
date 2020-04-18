CREATE TABLE kombucha (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    added TIMESTAMPTZ NOT NULL
);

CREATE TABLE kombucha_entry (
    id SERIAL PRIMARY KEY,
    kombucha_id INTEGER NOT NULL REFERENCES kombucha(id),
    content TEXT NOT NULL,
    added TIMESTAMPTZ NOT NULL
);
