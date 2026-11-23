--!strict
local task = require("std:task")
local clock = task.Clock.new()
clock:wait(1)
assert(clock:wait(1) == 2)
