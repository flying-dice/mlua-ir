--!strict
-- `decode` returns `unknown`: it must be checked before it is indexed.
local json = require("lib:json")
local value = assert(json.decode("{}"))
print(value.field)
