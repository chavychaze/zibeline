CREATE TYPE user_role AS ENUM ('super_admin', 'admin', 'user');

CREATE TABLE IF NOT EXISTS user_roles (
    id SERIAL PRIMARY KEY,
    role user_role NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    CONSTRAINT users_role_unique UNIQUE (role)
);