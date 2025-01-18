#!/bin/bash

# Create directory structure
mkdir -p scripts/lib scripts/commands

# Ensure files exist
touch scripts/lib/utils.lua scripts/lib/config.lua
touch scripts/commands/api.lua scripts/commands/cleanup.lua
touch scripts/commands/database.lua scripts/commands/ui.lua

# Make scripts executable
chmod +x scripts/main.lua

echo "✅ Script structure initialized"