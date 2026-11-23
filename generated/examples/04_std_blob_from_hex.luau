--!strict
local blob = require("std:blob")
local data = assert(blob.from_hex("00ff61"))
assert(data == "\0\255a")
local _, err = blob.from_hex("0")
assert(err ~= nil and err.kind == "odd_length")
