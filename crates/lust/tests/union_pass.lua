-- @type boolean?
local maybeFlag = nil

-- @type boolean | number | string
local flag = "hello" or true

-- @type string | number?
local value = nil or 456

-- @type nil | string | boolean
local text = "Hello world!"

-- @type boolean | string
local emptyAgain = maybeFlag or "text"
