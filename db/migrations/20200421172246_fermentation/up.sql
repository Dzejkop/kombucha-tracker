CREATE TYPE fermentation_status AS ENUM ('primary', 'secondary');

CREATE TABLE kombucha_fermentation (
    id SERIAL PRIMARY KEY,
    kombucha_id INTEGER NOT NULL REFERENCES kombucha(id),
    start_date TIMESTAMPTZ NOT NULL,
    end_date TIMESTAMPTZ,
    est_end_date TIMESTAMPTZ,
    status fermentation_status NOT NULL
);
