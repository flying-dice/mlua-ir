--!strict
local time = require("std:time")

local total = time.Duration.seconds(1):plus(time.Duration.seconds(2.5))
assert(total.as_seconds == 3.5)
assert(time.describe(total) == "3.5s")
assert(time.describe(0.25) == "0.25s")

-- a userdata is an extern type: `typeof` tells a union of objects apart, at check time and at run time
local timeout: std_time_Timeout = time.Duration.seconds(2)
if typeof(timeout) == "std_time_Duration" then
	assert(timeout.as_seconds == 2)
else
	error("expected a Duration")
end
