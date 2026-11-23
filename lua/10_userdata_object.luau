--!strict
local collections = require("std:collections")

local c = collections.Counter.new()
assert(c.value == 0)
c:increment()
assert(c:add(5) == 6)
c:set(-1)
assert(c.value == -1)
