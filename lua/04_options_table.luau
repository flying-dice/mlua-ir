--!strict
local dir = require("std:dir")

assert(#dir.list("src") == 1)
assert(#dir.list("src", { recursive = true }) == 3)
assert(#dir.list("src", { recursive = true, hidden = true }) == 4)
assert(#dir.list("src", { recursive = true, max_depth = 1 }) == 1)
