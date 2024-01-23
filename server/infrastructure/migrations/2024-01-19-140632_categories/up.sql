CREATE TABLE
    IF NOT EXISTS category (
        id SERIAL PRIMARY KEY,
        language INTEGER REFERENCES language(id),
        name VARCHAR(255) NOT NULL,
        image VARCHAR(255),
        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        is_delete BOOLEAN DEFAULT FALSE
    );