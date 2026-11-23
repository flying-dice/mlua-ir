--!strict
local collections = require("std:collections")
local c = collections.Counter.new(41)
assert(c:increment() == 42 and c.value == 42)
