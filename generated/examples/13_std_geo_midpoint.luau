--!strict
local geo = require("std:geo")
local m = geo.midpoint({ x = 0, y = 0 }, { x = 4, y = 2 })
assert(m.x == 2 and m.y == 1 and m.label == nil)
