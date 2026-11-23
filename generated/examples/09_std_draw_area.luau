--!strict
local draw = require("std:draw")
assert(draw.area({ kind = "rect", w = 2, h = 3 }) == 6)
