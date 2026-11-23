--!strict
local text = require("std:text")

assert(text.pad("id", 6) == "id    ")
assert(text.pad("longer", 2) == "longer")
