--!strict
-- bollard's `ContainerSummary` has `Names`, not `Nmaes`.
local docker_models = require("vnd:docker_models")
print(docker_models.sample()[1].Nmaes)
