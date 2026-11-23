--!strict
local bollard = require("vnd:bollard")

local docker, connect_err = bollard.connect({ host = "unix:///var/run/docker.sock" })
if not docker then
	error(connect_err)
end
assert(docker.host == "unix:///var/run/docker.sock")

assert(#assert(docker:list_containers()) == 1)
assert(#assert(docker:list_containers({ all = true })) == 2)

local missing, find_err = docker:find_container("nope")
assert(missing == nil and find_err == nil) -- absence, not an error

local id, create_err = docker:create_container("api", {
	Image = "nginx:1.27",
	Labels = { app = "api" },
	HostConfig = { RestartPolicy = { Name = "unless-stopped" } },
})
if not id then
	error(create_err)
end
assert(#id > 0)
local _, conflict = docker:create_container("api", { Image = "nginx:1.27" })
assert(conflict ~= nil and conflict.kind == "conflict")

local lines: { string } = {}
for piece in docker:logs("web", { tail = 2 }):items() do
	table.insert(lines, piece.data)
end
assert(#lines == 2 and lines[2] == "GET / 200")

local _, unreachable = bollard.connect({ host = "tcp://nowhere:1" })
assert(unreachable ~= nil and unreachable.kind == "connect")
