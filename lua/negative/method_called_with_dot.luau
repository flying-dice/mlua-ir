--!strict
-- A method needs its `self`: `c:increment()`.
local collections = require("std:collections")
local c = collections.Counter.new()
print(c.increment())
