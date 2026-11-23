--!strict
local time = require("std:time")

local total = time.Duration.seconds(1):plus(time.Duration.seconds(2.5))
assert(total.as_seconds == 3.5)
assert(time.describe(total) == "3.5s")
assert(time.describe(0.25) == "0.25s")
