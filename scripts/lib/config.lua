local M = {}

-- Load environment variables from .env file
function M.load_env()
    local env_file = io.open("../.env", "r")
    if not env_file then return {} end

    local env = {}
    for line in env_file:lines() do
        local key, value = line:match("^([^=]+)=(.+)$")
        if key and value then
            -- Remove quotes if present
            value = value:gsub('^"(.*)"$', '%1')
            value = value:gsub("^'(.*)'$", '%1')
            env[key] = value
        end
    end
    env_file:close()
    return env
end

-- Default configuration that can be overridden by .env
M.DEFAULT_CONFIG = {
    -- Database
    DATABASE_URL = "postgres://user:password@localhost/zibelina",
    POSTGRES_USER = "user",
    POSTGRES_PASSWORD = "password",
    POSTGRES_DB = "zibelina",
    POSTGRES_PORT = "5432",
    
    -- API
    API_PORT = "8000",
    RUST_LOG = "debug",
    
    -- UI
    UI_PORT = "80",
    API_URL = "http://localhost:8000",
    TRUNK_SERVE_PORT = "8080",
    
    -- General
    ENVIRONMENT = "development"
}

-- Get configuration with .env overrides
function M.get_config()
    local env = M.load_env()
    local config = {}
    for k, v in pairs(M.DEFAULT_CONFIG) do
        config[k] = env[k] or v
    end
    return config
end

-- Terminal colors
M.COLORS = {
    red = "\27[31m",
    green = "\27[32m",
    yellow = "\27[33m",
    blue = "\27[34m",
    reset = "\27[0m"
}

-- Service names
M.SERVICES = {
    UI = "zibelina-store-ui",
    API = "zibelina-store-api",
    DB = "zibelina-store-pg"
}

return M