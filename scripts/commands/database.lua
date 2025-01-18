local utils = require("lib.utils")
local config = require("lib.config")
local postgres_data = require("lib.postgres_data")

local M = {}

-- Check if PostgreSQL is ready
local function wait_for_postgres()
    utils.print_color("yellow", "⏳ Waiting for PostgreSQL to be ready...")
    while true do
        local result = utils.execute_command([[
            docker-compose exec -T zibelina-store-pg pg_isready -U user -d zibelina
        ]])
        if result:match("accepting connections") then
            break
        end
        utils.print_color("blue", "⏳ Database is not ready yet... waiting")
        os.execute("sleep 2")
    end
    utils.print_color("green", "✅ PostgreSQL is ready!")
end

-- Initialize database
function M.init()
    postgres_data.check()
    wait_for_postgres()
end

-- Run migrations
function M.migrate()
    M.init()
    local cfg = config.get_config()
    utils.print_color("blue", "🔄 Running database migrations...")
    
    local migration_result = utils.execute_command(string.format([[
        cd ../server && DATABASE_URL='%s' diesel migration run
    ]], cfg.DATABASE_URL))
    
    utils.print_color("green", migration_result)
end

-- Run seeds
function M.seed()
    M.init()
    local cfg = config.get_config()
    utils.print_color("blue", "🌱 Seeding database...")
    
    -- Get all seed files in order from infrastructure/seeds
    local seed_files = utils.execute_command([[
        cd ../server/infrastructure/seeds && ls -1 *.sql | sort
    ]])
    
    -- Execute each seed file
    for file in string.gmatch(seed_files, "[^\n]+") do
        utils.print_color("yellow", "Running seed file: " .. file)
        local seed_result = utils.execute_command(string.format([[
            PGPASSWORD='%s' psql -h localhost -U %s -d %s -f ../server/infrastructure/seeds/%s
        ]], cfg.POSTGRES_PASSWORD, cfg.POSTGRES_USER, cfg.POSTGRES_DB, file))
        utils.print_color("green", seed_result)
    end
end

-- Reset database
function M.reset()
    postgres_data.clean()
    M.init()
    M.migrate()
    M.seed()
end

-- Setup database (both migrate and seed)
function M.setup()
    M.init()
    M.migrate()
    M.seed()
end

-- Check database health
function M.health()
    local cfg = config.get_config()
    utils.print_color("blue", "🏥 Checking database health...")
    
    local health_check = utils.execute_command(string.format([[
        PGPASSWORD='%s' psql -h localhost -U %s -d %s -c "SELECT 1"
    ]], cfg.POSTGRES_PASSWORD, cfg.POSTGRES_USER, cfg.POSTGRES_DB))
    
    if health_check:match("1") then
        utils.print_color("green", "Database is healthy")
    else
        utils.print_color("red", "Database health check failed")
    end
end

return M