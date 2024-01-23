CREATE TABLE
    IF NOT EXISTS delivery_address (
        id SERIAL PRIMARY KEY,
        first_name VARCHAR(100),
        middle_name VARCHAR(100),
        last_name VARCHAR(100),
        country VARCHAR(100) NOT NULL,
        city VARCHAR(100) NOT NULL,
        region VARCHAR(100) NOT NULL,
        street VARCHAR(100) NOT NULL,
        building_number VARCHAR(100) NOT NULL,
        address VARCHAR(255),
        phone_number VARCHAR(100),
        comment VARCHAR(255),
        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
    );