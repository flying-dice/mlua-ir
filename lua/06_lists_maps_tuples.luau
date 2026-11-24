--!strict
local labels = require("std:labels")

local map = assert(labels.parse({ "app=web", "tier=front" })) -- raises the typed error table as it is
assert(map.app == "web" and map.tier == "front")

local keys = labels.keys(map)
assert(#keys == 2 and keys[1] == "app")

local first, count = labels.summary(map)
assert(first == "app" and count == 2)

local bad, why = labels.parse({ "oops" })
assert(bad == nil and why ~= nil and why.kind == "no_equals" and why.line == 1)
assert(tostring(why) == "std:labels.parse: no_equals: line 1 has no `=`: oops")

-- a variadic last parameter keeps a trailing nil
assert(labels.count() == 0 and labels.count("a", nil) == 2)
