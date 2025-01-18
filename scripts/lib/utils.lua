local M = {}

-- Execute shell commands and capture output
function M.execute_command(command)
    local handle = io.popen(command)
    local result = handle:read("*a")
    handle:close()
    return result
end

-- Print colored output
function M.print_color(color, message)
    local colors = {
        red = "\27[31m",
        green = "\27[32m",
        yellow = "\27[33m",
        blue = "\27[34m",
        reset = "\27[0m"
    }
    print(colors[color] .. message .. colors.reset)
end

return M