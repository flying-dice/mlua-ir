--!strict
local blob = require("std:blob")

local raw = "\0\1\254\255"
assert(blob.to_hex(raw) == "0001feff")
local back, err = blob.from_hex("0001feff")
assert(err == nil and back ~= nil and back == raw and #back == 4)

local _, odd = blob.from_hex("abc")
if odd ~= nil and odd.kind == "odd_length" then
	assert(odd.len == 3)
else
	error("expected odd_length")
end

assert(blob.global_type("print") == "function")
assert(blob.global_type_later("print") == "function")

local label = blob.Label.new("hi")
assert(tostring(label) == "label:hi")
assert(label:text_later() == "hi")
