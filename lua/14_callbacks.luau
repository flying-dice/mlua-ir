--!strict
local sort = require("std:sort")

local sorted = sort.sort_by({ "b", "c", "a" }, function(left, right)
	return left < right
end)
assert(table.concat(sorted, ",") == "a,b,c")

local seen: { string } = {}
local count = sort.each({ "x", "y" }, function(item, index)
	seen[index] = item
end)
assert(count == 2 and seen[2] == "y")

local ok, err = pcall(sort.sort_by, { "a", "b" }, function(): boolean
	error("no order today")
end)
assert(not ok and string.find(tostring(err), "no order today", 1, true) ~= nil)
