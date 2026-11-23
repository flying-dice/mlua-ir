--!strict
local events = require("std:events")

local ticks = events.count_to(3)
local seen: { number } = {}
while true do
	local tick, err = ticks:next()
	assert(err == nil)
	if tick == nil then
		break
	end
	table.insert(seen, tick.n)
end
assert(#seen == 3 and seen[3] == 3)

local failing = events.failing(2)
local got = 0
local stopped: std_events_EventsError? = nil
while true do
	local tick, err = failing:next()
	if err ~= nil then
		stopped = err
		break
	end
	if tick == nil then
		break
	end
	got += 1
end
assert(got == 2 and stopped ~= nil and stopped.after == 2)

local ok, raised = pcall(function()
	for _ in events.failing(1):items() do
	end
end)
assert(not ok and string.find(tostring(raised), "interrupted after 1 ticks", 1, true) ~= nil)
