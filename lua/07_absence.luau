--!strict
local env = require("std:env")

assert(env.var("HOME") == "/home/demo")
assert(env.var("NOPE") == nil)
assert(env.var_or("NOPE", "x") == "x")
assert(env.var_or("NOPE") == "")
