CREATE TABLE
    IF NOT EXISTS users (
        id SERIAL PRIMARY KEY,
        language INTEGER REFERENCES language(id),
        first_name VARCHAR(100),
        last_name VARCHAR(100),
        email VARCHAR(100),
        email_verified TIMESTAMP,
        role VARCHAR(15),
        image VARCHAR(100),
        phone_number VARCHAR(15),
        user_info VARCHAR(255),
        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        last_order TIMESTAMP,
        is_delete BOOLEAN NOT NULL DEFAULT FALSE
    );