CREATE TABLE
    IF NOT EXISTS transaction (
        id SERIAL PRIMARY KEY,
        user_id INTEGER REFERENCES users(id) NOT NULL,
        amount INTEGER NOT NULL,
        currency VARCHAR(15) NOT NULL,
        transaction_type VARCHAR(50) NOT NULL,
        status VARCHAR(50) NOT NULL,
        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP NOT NULL
    );