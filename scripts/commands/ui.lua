local utils = require("utils")
local config = require("config")

local M = {}

-- Build the UI
function M.build()
    utils.print_color("blue", "🔨 Building UI...")
    local result = utils.execute_command([[
        cd ../ui && \
        trunk build --release
    ]])
    utils.print_color("green", result)
end

-- Start UI development server
function M.dev()
    utils.print_color("blue", "🚀 Starting UI development server...")
    utils.execute_command([[
        cd ../ui && \
        trunk serve
    ]])
end

-- Install UI dependencies
function M.install()
    utils.print_color("blue", "📦 Installing UI dependencies...")
    
    -- Check if trunk is installed
    local trunk_exists = utils.execute_command("command -v trunk")
    if trunk_exists == "" then
        utils.print_color("yellow", "Installing trunk...")
        utils.execute_command("cargo install trunk wasm-bindgen-cli")
    end
    
    -- Add wasm target
    utils.execute_command("rustup target add wasm32-unknown-unknown")
    
    utils.print_color("green", "✅ UI dependencies installed")
end

-- Check UI health
function M.health()
    local cfg = config.get_config()
    utils.print_color("blue", "🏥 Checking UI health...")
    
    local health_check = utils.execute_command(
        string.format("curl -s http://localhost:%s", cfg.UI_PORT)
    )
    
    if health_check ~= "" then
        utils.print_color("green", "UI is responding")
    else
        utils.print_color("red", "UI is not responding")
    end
end

-- Get UI logs
function M.logs()
    local cfg = config.get_config()
    utils.print_color("blue", "📜 Fetching UI logs...")
    
    if cfg.ENVIRONMENT == "docker" then
        utils.execute_command("cd .. && docker-compose logs --tail=100 -f zibelina-store-ui")
    else
        utils.print_color("yellow", "Local UI logs not available in development mode")
    end
end

return M