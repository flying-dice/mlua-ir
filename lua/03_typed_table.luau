--!strict
local geo = require("std:geo")

local a: std_geo_Point = { x = 0, y = 0 }
local b: std_geo_Point = { x = 4, y = 2, label = "b" }
local m = geo.midpoint(a, b)
assert(m.x == 2 and m.y == 1 and m.label == nil)
assert(geo.distance(a, { x = 3, y = 4 }) == 5)
