CREATE TABLE
    IF NOT EXISTS delivery_service (
        id SERIAL PRIMARY KEY,
        delivery_service_details VARCHAR(255) NOT NULL,
        name VARCHAR(100),
        country VARCHAR(100) NOT NULL,
        phone_number VARCHAR(100),
        comment VARCHAR(255),
        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
    );