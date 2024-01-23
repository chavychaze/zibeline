CREATE TABLE
    IF NOT EXISTS rating (
        id SERIAL PRIMARY KEY,
        user_id INTEGER REFERENCES users(id),
        product_id INTEGER REFERENCES product(id),
        user_rating INTEGER NOT NULL,
        comment VARCHAR(255),
        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
    );