--!strict
local blob = require("std:blob")
assert(blob.global_type_later("print") == "function")
