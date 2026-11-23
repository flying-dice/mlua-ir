--!strict
local time = require("std:time")
assert(time.describe(1.5) == "1.5s")
assert(time.describe(time.Duration.seconds(2)) == "2s")
