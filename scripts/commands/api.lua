local utils = require("utils")
local config = require("config")

local M = {}

-- Build the API
function M.build()
    utils.print_color("blue", "🔨 Building API...")
    local cfg = config.get_config()
    local build_result = utils.execute_command(
        string.format(
            "cd ../server && DATABASE_URL='%s' cargo build --release",
            cfg.DATABASE_URL
        )
    )
    utils.print_color("green", build_result)
end

-- Start the API
function M.start()
    local cfg = config.get_config()
    utils.print_color("blue", "🚀 Starting API...")
    
    -- Check if we're in Docker environment
    if cfg.ENVIRONMENT == "docker" then
        utils.execute_command("cd .. && docker-compose up -d zibelina-store-api")
    else
        -- Run locally
        utils.execute_command(
            string.format(
                "cd ../server && DATABASE_URL='%s' RUST_LOG='%s' cargo run --release",
                cfg.DATABASE_URL,
                cfg.RUST_LOG
            )
        )
    end
end

-- Stop the API
function M.stop()
    utils.print_color("blue", "🛑 Stopping API...")
    local cfg = config.get_config()
    
    if cfg.ENVIRONMENT == "docker" then
        utils.execute_command("cd .. && docker-compose stop zibelina-store-api")
    else
        -- Find and kill the local process
        local pid = utils.execute_command("pgrep -f 'zibelina-store'")
        if pid ~= "" then
            utils.execute_command("kill " .. pid)
        end
    end
end

-- Check API status
function M.status()
    local cfg = config.get_config()
    utils.print_color("blue", "📊 Checking API status...")
    
    if cfg.ENVIRONMENT == "docker" then
        local status = utils.execute_command("cd .. && docker-compose ps zibelina-store-api")
        utils.print_color("green", status)
    else
        local pid = utils.execute_command("pgrep -f 'zibelina-store'")
        if pid ~= "" then
            utils.print_color("green", "API is running with PID: " .. pid)
        else
            utils.print_color("yellow", "API is not running")
        end
    end
end

-- Health check
function M.health()
    local cfg = config.get_config()
    utils.print_color("blue", "🏥 Checking API health...")
    
    local health_check = utils.execute_command(
        string.format("curl -s http://localhost:%s/health", cfg.API_PORT)
    )
    
    if health_check:match("ok") then
        utils.print_color("green", "API is healthy")
    else
        utils.print_color("red", "API health check failed")
    end
end

-- Logs
function M.logs()
    local cfg = config.get_config()
    utils.print_color("blue", "📜 Fetching API logs...")
    
    if cfg.ENVIRONMENT == "docker" then
        utils.execute_command("cd .. && docker-compose logs --tail=100 -f zibelina-store-api")
    else
        utils.execute_command("tail -f ../server/logs/zibelina.log")
    end
end

return M