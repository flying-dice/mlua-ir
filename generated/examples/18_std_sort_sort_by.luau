--!strict
local sort = require("std:sort")
local sorted = sort.sort_by({ "pear", "fig", "apple" }, function(left, right)
    return #left < #right
end)
assert(sorted[1] == "fig" and sorted[3] == "apple")
