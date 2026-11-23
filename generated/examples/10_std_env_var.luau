--!strict
local env = require("std:env")
assert(env.var("HOME") == "/home/demo")
assert(env.var("NOPE") == nil)
