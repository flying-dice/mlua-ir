--!strict
local bollard = require("vnd:bollard")
local docker = assert(bollard.connect())
for _, c in assert(docker:list_containers({ all = true })) do
    assert(c.Names ~= nil)
end
