CREATE TABLE
    IF NOT EXISTS registration_confirmation (
        id SERIAL PRIMARY KEY,
        user_id INTEGER REFERENCES users(id) NOT NULL,
        activation_token VARCHAR(255) NOT NULL,
        expired_at TIMESTAMP
    );