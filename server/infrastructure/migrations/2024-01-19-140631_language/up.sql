CREATE TABLE
    IF NOT EXISTS language (
        id SERIAL PRIMARY KEY,
        short_name VARCHAR(15),
        full_name VARCHAR(100),
        image VARCHAR(255)
    );