-- ===== vnd:docker_models =====

export type vnd_docker_models_ContainerSummary = {
	Id: string?,
	Names: { string }?,
	Image: string?,
	ImageID: string?,
	ImageManifestDescriptor: vnd_docker_models_OciDescriptor?,
	Command: string?,
	Created: number?,
	Ports: { vnd_docker_models_PortSummary }?,
	SizeRw: number?,
	SizeRootFs: number?,
	Labels: { [string]: string }?,
	State: vnd_docker_models_ContainerSummaryStateEnum?,
	Status: string?,
	HostConfig: vnd_docker_models_ContainerSummaryHostConfig?,
	NetworkSettings: vnd_docker_models_ContainerSummaryNetworkSettings?,
	Mounts: { vnd_docker_models_MountPoint }?,
	Health: vnd_docker_models_ContainerSummaryHealth?,
}

export type vnd_docker_models_ContainerSummaryHealth = {
	Status: vnd_docker_models_ContainerSummaryHealthStatusEnum?,
	FailingStreak: number?,
}

export type vnd_docker_models_ContainerSummaryHealthStatusEnum = "" | "none" | "starting" | "healthy" | "unhealthy"

export type vnd_docker_models_ContainerSummaryHostConfig = {
	NetworkMode: string?,
	Annotations: { [string]: string }?,
}

export type vnd_docker_models_ContainerSummaryNetworkSettings = {
	Networks: { [string]: vnd_docker_models_EndpointSettings }?,
}

export type vnd_docker_models_ContainerSummaryStateEnum = "" | "created" | "running" | "paused" | "restarting" | "exited" | "removing" | "dead"

export type vnd_docker_models_EndpointIpamConfig = {
	IPv4Address: string?,
	IPv6Address: string?,
	LinkLocalIPs: { string }?,
}

export type vnd_docker_models_EndpointSettings = {
	IPAMConfig: vnd_docker_models_EndpointIpamConfig?,
	Links: { string }?,
	MacAddress: string?,
	Aliases: { string }?,
	DriverOpts: { [string]: string }?,
	GwPriority: number?,
	NetworkID: string?,
	EndpointID: string?,
	Gateway: string?,
	IPAddress: string?,
	IPPrefixLen: number?,
	IPv6Gateway: string?,
	GlobalIPv6Address: string?,
	GlobalIPv6PrefixLen: number?,
	DNSNames: { string }?,
}

export type vnd_docker_models_MountPoint = {
	Type: vnd_docker_models_MountPointTypeEnum?,
	Name: string?,
	Source: string?,
	Destination: string?,
	Driver: string?,
	Mode: string?,
	RW: boolean?,
	Propagation: string?,
}

export type vnd_docker_models_MountPointTypeEnum = "" | "bind" | "volume" | "image" | "tmpfs" | "npipe" | "cluster"

export type vnd_docker_models_OciDescriptor = {
	mediaType: string?,
	digest: string?,
	size: number?,
	urls: { string }?,
	annotations: { [string]: string }?,
	data: string?,
	platform: vnd_docker_models_OciPlatform?,
	artifactType: string?,
}

export type vnd_docker_models_OciPlatform = {
	architecture: string?,
	os: string?,
	["os.version"]: string?,
	["os.features"]: { string }?,
	variant: string?,
}

export type vnd_docker_models_PortSummary = {
	IP: string?,
	PrivatePort: number,
	PublicPort: number?,
	Type: string?,
}

export type vnd_docker_models = {
	--- Two containers as Docker reports them: bollard's own `ContainerSummary`, typed field by field.
	---
	--- ```luau
	--- local containers = docker_models.sample()
	--- assert(containers[1].State == "running")
	--- ```
	sample: () -> { vnd_docker_models_ContainerSummary },
	--- One line for a container: its first name and its state. Takes bollard's type as an argument too.
	line: (container: vnd_docker_models_ContainerSummary) -> string,
}
