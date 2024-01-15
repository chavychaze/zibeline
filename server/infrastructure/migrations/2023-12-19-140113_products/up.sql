CREATE TABLE
    IF NOT EXISTS products (
        id SERIAL PRIMARY KEY,
        name VARCHAR(255) NOT NULL,
        description TEXT,
        price INTEGER NOT NULL,
        pieces INTEGER NOT NULL,
        pieces_left INTEGER NOT NULL,
        group_ids INTEGER[] NOT NULL,
        last_order TIMESTAMP NOT NULL,
        created_at TIMESTAMP NOT NULL,
        updated_at TIMESTAMP NOT NULL,
        is_delete BOOLEAN DEFAULT FALSE
    );