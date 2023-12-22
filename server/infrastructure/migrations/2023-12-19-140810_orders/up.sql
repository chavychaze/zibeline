CREATE TABLE
    IF NOT EXISTS orders (
        id SERIAL PRIMARY KEY,
        user_id INTEGER REFERENCES users(id),
        product_ids INTEGER[] NOT NULL,
        created_at TIMESTAMP NOT NULL,
        updated_at TIMESTAMP NOT NULL,
        is_formed BOOLEAN DEFAULT FALSE,
        is_sent BOOLEAN DEFAULT FALSE,
        is_delivered BOOLEAN NOT NULL DEFAULT FALSE,
        is_delete BOOLEAN NOT NULL DEFAULT FALSE
    );