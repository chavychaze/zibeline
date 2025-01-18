#!/bin/bash

# Check if name argument is provided
if [ -z "$1" ]; then
    echo "Usage: $0 <seed_name>"
    exit 1
fi

# Create seeds directory if it doesn't exist
mkdir -p server/infrastructure/seeds

# Generate timestamp
timestamp=$(date +%Y%m%d%H%M%S)

# Create seed file name
seed_file="server/infrastructure/seeds/${timestamp}_${1}.sql"

# Create seed file with template
cat > "$seed_file" << EOF
-- Seed: ${1}
-- Created at: $(date)

BEGIN;

-- Add your seed data here
-- Example:
-- INSERT INTO table_name (column1, column2) VALUES ('value1', 'value2');

COMMIT;
EOF

echo "Created seed file: $seed_file"