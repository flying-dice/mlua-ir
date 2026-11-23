--!strict
local log = require("std:log")
assert(log.write("warn", "disk nearly full") == "[warn] disk nearly full")
