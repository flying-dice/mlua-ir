--!strict
local task = require("std:task")

assert(task.sleep(2) == 2)
local clock = task.Clock.new()
for _ = 1, 3 do
	clock:wait(1)
end
assert(clock:wait(0) == 4)
