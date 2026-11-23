--!strict
local fs = require("std:fs")
local text, err = fs.read("notes.txt")
assert(err == nil and text == "hello")
local missing, why = fs.read("nope.txt")
assert(missing == nil and why ~= nil and why.kind == "not_found")
