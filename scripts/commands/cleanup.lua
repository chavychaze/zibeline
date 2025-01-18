local utils = require("lib.utils")
local config = require("lib.config")
local postgres_data = require("lib.postgres_data")

local M = {}

function M.execute()
    utils.print_color("blue", "🧹 Cleaning up environment...")
    
    -- Stop all containers
    utils.print_color("yellow", "Stopping containers...")
    utils.execute_command("cd .. && docker-compose down")
    
    -- Clean postgres data
    postgres_data.clean()
    
    -- Clean UI builds
    utils.print_color("yellow", "Cleaning UI builds...")
    utils.execute_command("cd ../ui && rm -rf dist")
    
    -- Clean Rust builds
    utils.print_color("yellow", "Cleaning Rust builds...")
    utils.execute_command("cd ../server && cargo clean")
    utils.execute_command("cd ../ui && cargo clean")
    
    utils.print_color("green", "✨ Cleanup complete")
end

return M