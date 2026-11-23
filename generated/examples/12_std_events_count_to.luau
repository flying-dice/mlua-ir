--!strict
local events = require("std:events")
local seen = 0
for tick in events.count_to(3):items() do
    seen += tick.n
end
assert(seen == 6)
