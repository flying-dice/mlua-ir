--!strict
local names = require("std:names")

assert(names.describe("web-1") == "container web-1")
local ok, err = pcall(names.describe, "-bad")
assert(not ok and string.find(tostring(err), "is not a container name", 1, true) ~= nil)
