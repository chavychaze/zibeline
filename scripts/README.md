# Scripts Directory

This directory contains Lua scripts for managing the Zibelina Store application deployment, database migrations, and server setup.

## Structure

```
scripts/
├── commands/           # Individual command modules
│   ├── api.lua        # API management commands
│   ├── cleanup.lua    # Cleanup operations
│   └── database.lua   # Database operations
├── config.lua         # Configuration management
├── main.lua          # Main script entry point
├── utils.lua         # Utility functions
└── server_setup.sh   # Server provisioning script
```

## Prerequisites

- Lua 5.3 or later
- PostgreSQL 16.1
- Docker and Docker Compose (optional)

## Server Setup

### Option 1: Automated Setup

1. SSH into your server
2. Copy the `server_setup.sh` script:
```bash
scp scripts/server_setup.sh user@your-server:/tmp/
```

3. Run the setup script:
```bash
ssh user@your-server "chmod +x /tmp/server_setup.sh && sudo /tmp/server_setup.sh"
```

### Option 2: Manual Setup

1. Install required packages:
```bash
sudo apt update
sudo apt install -y postgresql-16 nginx certbot python3-certbot-nginx
```

2. Setup PostgreSQL:
```bash
sudo -u postgres createuser --pwprompt zibelina
sudo -u postgres createdb --owner=zibelina zibelina_db
```

3. Configure Nginx:
```bash
sudo nano /etc/nginx/sites-available/zibelina
# Copy the configuration from server_setup.sh
sudo ln -s /etc/nginx/sites-available/zibelina /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl reload nginx
```

4. Setup SSL:
```bash
sudo certbot --nginx -d your-domain.com
```

## Script Usage

### Environment Setup

1. Create `.env` file in project root:
```bash
cp .env.example .env
```

2. Update variables in `.env`:
```env
DATABASE_URL=postgres://user:password@localhost/zibelina
POSTGRES_USER=user
POSTGRES_PASSWORD=password
POSTGRES_DB=zibelina
API_PORT=8000
ENVIRONMENT=development
```

### Running Scripts

All scripts should be run from the project root directory:

```bash
# Reset everything
./scripts/main.lua reset

# Run migrations only
./scripts/main.lua migrate

# Run seeds only
./scripts/main.lua seed

# Start API
./scripts/main.lua start

# Check API status
./scripts/main.lua status

# View logs
./scripts/main.lua logs
```

## Extending Scripts

### Adding New Commands

1. Create new command file in `commands/`:
```lua
-- commands/new_command.lua
local utils = require("utils")
local config = require("config")

local M = {}

function M.execute()
    -- Command implementation
end

return M
```

2. Add command to `main.lua`:
```lua
local commands = {
    new_command = require("commands.new_command").execute
}
```