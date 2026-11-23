--!strict
local json = require("lib:json")

local value, err = json.decode('{"n": 1, "list": [1, 2]}')
assert(err == nil)
if type(value) == "table" then
	local object = value :: { [string]: unknown } -- checked above: it is a table
	assert(object.n == 1)
else
	error("expected a table")
end
assert(json.encode({ a = true }) == '{"a":true}')

local _, bad = json.decode("{")
assert(bad ~= nil and bad.kind == "syntax")
