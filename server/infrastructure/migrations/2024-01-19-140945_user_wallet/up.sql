CREATE TABLE
    IF NOT EXISTS user_wallet (
        id SERIAL PRIMARY KEY,
        user_id INTEGER REFERENCES users(id),
        last_transaction INTEGER REFERENCES transaction(id),
        currency VARCHAR(50) NOT NULL,
        amount INTEGER NOT NULL,
        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP NOT NULL
    );