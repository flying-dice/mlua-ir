--!strict
local parse = require("std:parse")

assert(parse.number(" 42 ") == 42)

local ok, caught = pcall(parse.number, "")
assert(not ok)
local err = parse.error_of(caught)
assert(err ~= nil and err.kind == "empty")
assert(tostring(err) == "std:parse.number: empty: the text is empty")

local ok2, other = pcall(error, "plain")
assert(not ok2 and parse.error_of(other) == nil)
