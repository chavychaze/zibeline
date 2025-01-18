#!/usr/bin/env lua

-- Add scripts directory to Lua path
local script_path = debug.getinfo(1).source:match("@?(.*/)"):sub(1, -1)
package.path = script_path .. "?.lua;" .. script_path .. "lib/?.lua;" .. package.path

-- Initialize modules
local utils = require("lib.utils")
local config = require("lib.config")
local cleanup = require("commands.cleanup")
local database = require("commands.database")
local api = require("commands.api")
local ui = require("commands.ui")

-- Command handlers
local command_handlers = {
    -- Reset everything
    reset = function()
        cleanup.execute()
        database.setup()
        api.start()
        ui.build()
    end,
    
    -- Database commands
    migrate = function()
        database.setup()
    end,
    
    seed = function()
        database.seed()
    end,
    
    -- API commands
    ["api"] = {
        start = function()
            api.start()
        end,
        build = function()
            api.build()
        end,
        logs = function()
            api.logs()
        end
    },
    
    -- UI commands
    ["ui"] = {
        start = function()
            ui.dev()
        end,
        build = function()
            ui.build()
        end,
        install = function()
            ui.install()
        end,
        logs = function()
            ui.logs()
        end
    },
    
    -- Development mode
    dev = function()
        database.setup()
        api.start()
        ui.dev()
    end,
    
    -- Health checks
    health = function()
        database.health()
        api.health()
        ui.health()
    end
}

local function print_usage()
    utils.print_color("yellow", [[
Usage: ./scripts/main.lua <command> [subcommand]

Commands:
    reset               Reset everything (clean, migrate, seed, start)
    migrate             Run database migrations
    seed                Seed the database
    dev                 Start development environment
    health              Check health of all services

API Commands:
    api start          Start API server
    api build          Build API
    api logs           View API logs

UI Commands:
    ui start           Start UI development server
    ui build           Build UI for production
    ui install         Install UI dependencies
    ui logs            View UI logs
]])
end

local function main()
    local command = arg[1]
    local subcommand = arg[2]
    
    if not command then
        print_usage()
        os.exit(1)
    end
    
    local cmd = command_handlers[command]
    if type(cmd) == "table" and subcommand then
        cmd = cmd[subcommand]
    end
    
    if not cmd then
        utils.print_color("red", "Unknown command: " .. command .. (subcommand and " " .. subcommand or ""))
        print_usage()
        os.exit(1)
    end
    
    local success, error_message = pcall(cmd)
    
    if not success then
        utils.print_color("red", "❌ Error occurred: " .. tostring(error_message))
        os.exit(1)
    end
    
    utils.print_color("green", "✨ Done!")
end

main()