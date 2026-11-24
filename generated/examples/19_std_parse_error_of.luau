--!strict
local parse = require("std:parse")
local ok, caught = pcall(parse.number, "x")
assert(not ok)
local err = parse.error_of(caught)
assert(err ~= nil and err.kind == "not_a_number")
