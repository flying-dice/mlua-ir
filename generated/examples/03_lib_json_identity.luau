--!strict
local json = require("lib:json")
local t = {}
assert(json.identity(t) == t)
