--!strict
local math2 = require("std:math2")

assert(type(VOS_VERSION) == "string" and #VOS_VERSION > 0)
assert(MAX_ARGS == 64)
assert(uptime() >= 0)
assert(math.abs(math2.TAU - 2 * math.pi) < 1e-12)
