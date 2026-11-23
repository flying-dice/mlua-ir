--!strict
local blob = require("std:blob")
assert(blob.to_hex("\0\255a") == "00ff61")
