CREATE TABLE
    IF NOT EXISTS orders (
        id SERIAL PRIMARY KEY,
        user_id INTEGER REFERENCES users(id),
        delivery_address_id INTEGER REFERENCES delivery_address(id),
        delivery_service_id INTEGER REFERENCES delivery_service(id),
        transaction INTEGER REFERENCES transaction(id),
        product_ids INTEGER[] NOT NULL,
        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        status VARCHAR(50) NOT NULL
    );