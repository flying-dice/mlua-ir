--!strict
local text = require("std:text")

assert(text.pad("id", 6) == "id    ")
assert(text.pad("longer", 2) == "longer")

-- a Rust keyword as a name: `r#loop` is `loop`
assert(text["loop"]("ab", 2) == "abab")
