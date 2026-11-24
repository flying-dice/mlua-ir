--!strict
local docker_models = require("vnd:docker_models")

local containers = docker_models.sample()
assert(#containers == 2)
local web = containers[1]
assert(web.State == "running" and web.Names ~= nil and web.Names[1] == "/web")
assert(web.Labels ~= nil and web.Labels.app == "web")
assert(docker_models.line(web) == "/web running")
assert(docker_models.line({ Names = { "/x" }, State = "exited" }) == "/x exited")

-- a model with parsed dates (bollard `time`) traces too
assert(docker_models.inspect_sample().Id == "4f2a")
