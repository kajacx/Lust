-- @type number?
local maybeValue = nil

--@type string?
local maybeText = "Hello"

if maybeValue then
    -- @type number
    local valueInsideIf = maybeValue
end

-- @type number | string
local anotherValue = maybeValue or "error"

if maybeValue and maybeText then
    -- @type number
    local value = maybeValue

    -- @type string
    local text = maybeText
end
