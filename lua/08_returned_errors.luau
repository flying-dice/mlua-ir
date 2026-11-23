--!strict
local fs = require("std:fs")

local text, err = fs.read("notes.txt")
assert(err == nil and text == "hello")

local _, denied = fs.read("secret.txt")
assert(denied ~= nil and denied.kind == "permission_denied" and denied.path == "secret.txt")

local _, big = fs.read("huge.bin")
if big ~= nil and big.kind == "too_large" then
	assert(big.limit == 1024) -- only a `too_large` error has `limit`
else
	error("expected too_large")
end

assert(fs.remove("notes.txt") == true)
local removed, why = fs.remove("nope")
assert(removed == nil and why ~= nil and why.op == "std:fs.remove")
