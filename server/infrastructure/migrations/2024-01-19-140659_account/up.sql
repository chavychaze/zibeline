CREATE TABLE
    IF NOT EXISTS account (
        id SERIAL PRIMARY KEY,
        user_id INTEGER REFERENCES users(id) NOT NULL,
        account_type VARCHAR(255),
        provider VARCHAR(100),
        provider_user_id VARCHAR(255),
        access_token VARCHAR(255),
        refresh_token VARCHAR(255),
        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        expired_at TIMESTAMP
    );