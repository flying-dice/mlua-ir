--!strict
local json = require("lib:json")
local value, err = json.decode("{\"n\": 1}")
assert(err == nil)
if type(value) == "table" then
    local object = value :: { [string]: unknown } -- checked above: it is a table
    assert(object.n == 1)
end
