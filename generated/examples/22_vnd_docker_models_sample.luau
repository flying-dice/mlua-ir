--!strict
local docker_models = require("vnd:docker_models")
local containers = docker_models.sample()
assert(containers[1].State == "running")
