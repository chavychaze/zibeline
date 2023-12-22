CREATE TABLE
    IF NOT EXISTS users (
        id SERIAL PRIMARY KEY,
        name VARCHAR(100),
        email VARCHAR(100),
        address VARCHAR(255),
        phone VARCHAR(15),
        created_at TIMESTAMP NOT NULL,
        updated_at TIMESTAMP NOT NULL,
        last_order TIMESTAMP NOT NULL,
        is_registered BOOLEAN NOT NULL DEFAULT FALSE,
        is_delete BOOLEAN NOT NULL DEFAULT FALSE
    );