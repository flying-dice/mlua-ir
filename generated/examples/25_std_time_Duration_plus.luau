--!strict
local time = require("std:time")
local d = time.Duration.seconds(1):plus(time.Duration.seconds(0.5))
assert(d.as_seconds == 1.5)
