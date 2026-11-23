--!strict
-- Only a `too_large` error has `limit`; it is read before narrowing on `kind`.
local fs = require("std:fs")
local _, err = fs.read("x")
if err ~= nil then
	print(err.limit)
end
