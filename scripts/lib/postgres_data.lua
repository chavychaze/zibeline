local utils = require("lib.utils")
local config = require("lib.config")

local M = {}

-- Initialize postgres-data directory
function M.init()
    utils.print_color("blue", "🗂️  Initializing postgres-data directory...")
    utils.execute_command([[
        mkdir -p ../postgres-data
        chmod 777 ../postgres-data
    ]])
    utils.print_color("green", "✅ postgres-data directory initialized")
end

-- Check if postgres-data directory exists and is writable
function M.check()
    local dir_exists = utils.execute_command([[
        test -d ../postgres-data && echo "yes" || echo "no"
    ]])
    
    if dir_exists:match("no") then
        M.init()
    end
    
    -- Check permissions
    local is_writable = utils.execute_command([[
        test -w ../postgres-data && echo "yes" || echo "no"
    ]])
    
    if is_writable:match("no") then
        utils.print_color("yellow", "⚠️  Fixing postgres-data permissions...")
        utils.execute_command("chmod 777 ../postgres-data")
    end
end

-- Clean postgres-data directory
function M.clean()
    utils.print_color("blue", "🧹 Cleaning postgres-data directory...")
    utils.execute_command([[
        rm -rf ../postgres-data/*
        mkdir -p ../postgres-data
        chmod 777 ../postgres-data
    ]])
    utils.print_color("green", "✅ postgres-data directory cleaned")
end

return M