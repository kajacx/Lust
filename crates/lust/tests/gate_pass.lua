-- @type number?
local value = nil

if value then
    -- @type number
    local valueInsideIf = value
end

-- @type number | string
local anotherValue = value or "error"
