BEGIN;

-- Insert super admin user
INSERT INTO users (
    name,
    email,
    address,
    phone,
    is_registered,
    created_at,
    updated_at,
    last_order
) VALUES (
    'Super Admin',
    'super.admin@zibelina.com',
    'Admin Office',
    '+1234567890',
    true,
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP
);

-- Insert regular admin user
INSERT INTO users (
    name,
    email,
    address,
    phone,
    is_registered,
    created_at,
    updated_at,
    last_order
) VALUES (
    'Admin User',
    'admin@zibelina.com',
    'Admin Office',
    '+1234567891',
    true,
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP
);

-- Create temporary table to store user IDs
CREATE TEMPORARY TABLE temp_user_ids (
    name VARCHAR(100),
    id INTEGER
);

-- Insert sample users and store their IDs
WITH inserted_users AS (
    INSERT INTO users (
        name,
        email,
        address,
        phone,
        is_registered,
        created_at,
        updated_at,
        last_order
    ) VALUES
        (
            'John Doe',
            'john@example.com',
            '123 Main St',
            '+1234567890',
            true,
            CURRENT_TIMESTAMP,
            CURRENT_TIMESTAMP,
            CURRENT_TIMESTAMP
        ),
        (
            'Jane Smith',
            'jane@example.com',
            '456 Oak Ave',
            '+1987654321',
            true,
            CURRENT_TIMESTAMP,
            CURRENT_TIMESTAMP,
            CURRENT_TIMESTAMP
        ),
        (
            'Guest User',
            NULL,
            NULL,
            NULL,
            false,
            CURRENT_TIMESTAMP,
            CURRENT_TIMESTAMP,
            CURRENT_TIMESTAMP
        )
    RETURNING id, name
)
INSERT INTO temp_user_ids (id, name)
SELECT id, name FROM inserted_users;

-- Insert sample products
INSERT INTO products (
    name,
    description,
    price,
    pieces,
    pieces_left,
    group_ids,
    last_order,
    created_at,
    updated_at,
    is_delete
) VALUES
    (
        'Classic T-Shirt',
        'Comfortable cotton t-shirt',
        2500,
        100,
        95,
        '{1}',
        CURRENT_TIMESTAMP,
        CURRENT_TIMESTAMP,
        CURRENT_TIMESTAMP,
        false
    ),
    (
        'Designer Jeans',
        'Premium denim jeans',
        7500,
        50,
        48,
        '{1,2}',
        CURRENT_TIMESTAMP,
        CURRENT_TIMESTAMP,
        CURRENT_TIMESTAMP,
        false
    ),
    (
        'Summer Dress',
        'Light and breezy summer dress',
        4500,
        75,
        70,
        '{2}',
        CURRENT_TIMESTAMP,
        CURRENT_TIMESTAMP,
        CURRENT_TIMESTAMP,
        false
    ),
    (
        'Winter Coat',
        'Warm winter coat with fur lining',
        12000,
        30,
        28,
        '{3}',
        CURRENT_TIMESTAMP,
        CURRENT_TIMESTAMP,
        CURRENT_TIMESTAMP,
        false
    );

-- Insert sample orders using the stored user IDs
INSERT INTO orders (
    user_id,
    product_ids,
    is_formed,
    is_sent,
    is_delivered,
    is_delete,
    created_at,
    updated_at
)
SELECT 
    id,
    CASE name
        WHEN 'John Doe' THEN '{1,2}'::integer[]
        WHEN 'Jane Smith' THEN '{3}'::integer[]
        ELSE '{1}'::integer[]
    END,
    CASE name
        WHEN 'John Doe' THEN true
        WHEN 'Jane Smith' THEN true
        ELSE false
    END,
    CASE name
        WHEN 'John Doe' THEN true
        ELSE false
    END,
    false,
    false,
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP
FROM temp_user_ids;

-- Clean up
DROP TABLE temp_user_ids;

COMMIT;