CREATE TABLE
    IF NOT EXISTS product (
        id SERIAL PRIMARY KEY,
        language INTEGER REFERENCES language(id),
        category INTEGER[] NOT NULL,
        description INTEGER REFERENCES product_description(id),
        name VARCHAR(255) NOT NULL,
        image VARCHAR(255),
        weight INTEGER[] NOT NULL,
        price INTEGER NOT NULL,
        pieces INTEGER NOT NULL,
        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        is_delete BOOLEAN NOT NULL DEFAULT FALSE
    );