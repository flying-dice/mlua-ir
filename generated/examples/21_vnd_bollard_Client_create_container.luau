--!strict
local bollard = require("vnd:bollard")
local docker = assert(bollard.connect())
local id = assert(docker:create_container("api", { Image = "nginx:1.27", Labels = { app = "api" } }))
assert(docker:find_container("api") ~= nil and #id > 0)
