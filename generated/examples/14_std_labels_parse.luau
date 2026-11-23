--!strict
local labels = require("std:labels")
local map = assert(labels.parse({ "app=web", "tier=front" }))
assert(map.app == "web")
