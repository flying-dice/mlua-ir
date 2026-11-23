--!strict
local dir = require("std:dir")
assert(#dir.list("src") == 1)
assert(#dir.list("src", { recursive = true }) == 3)
assert(#dir.list("src", { recursive = true, hidden = true, max_depth = 2 }) == 3)
