-- ===== vnd:bollard =====

--- A client for one daemon.
declare extern type vnd_bollard_Client with
	--- The daemon this client talks to. Read-only.
	host: string
	--- The containers, as Docker's summaries. Only running ones unless `all`.
	---
	--- Yields the calling coroutine while it waits.
	list_containers: (self: vnd_bollard_Client, opts: vnd_bollard_ListContainers?) -> ({ vnd_bollard_ContainerSummary }?, vnd_bollard_DockerError?)
	--- One container's summary, or `nil` when there is none: absence is not an error.
	---
	--- Yields the calling coroutine while it waits.
	find_container: (self: vnd_bollard_Client, name: std_names_ContainerName) -> (vnd_bollard_ContainerSummary?, vnd_bollard_DockerError?)
	--- Create a container from Docker's own create body (`Image`, `Labels`, `Env`, `HostConfig`, ...): its id.
	---
	--- ```luau
	--- local docker = assert(bollard.connect())
	--- local id = assert(docker:create_container("api", { Image = "nginx:1.27", Labels = { app = "api" } }))
	--- assert(docker:find_container("api") ~= nil and #id > 0)
	--- ```
	---
	--- Yields the calling coroutine while it waits.
	create_container: (self: vnd_bollard_Client, name: std_names_ContainerName, body: vnd_bollard_ContainerCreateBody) -> (string?, vnd_bollard_DockerError?)
	--- A container's output as a stream; a container that does not exist is the stream's error.
	logs: (self: vnd_bollard_Client, name: std_names_ContainerName, opts: vnd_bollard_Logs?) -> mlua_Stream<vnd_bollard_LogPiece, vnd_bollard_DockerError>
end

--- How to reach a daemon.
export type vnd_bollard_ConnectOptions = {
	--- `unix:///var/run/docker.sock` or `tcp://host:2375`; the local socket when not given.
	host: string?,
	--- Seconds a call waits for the daemon.
	timeout: number?,
	--- Client certificates for a TLS daemon.
	tls: vnd_bollard_Tls?,
}

export type vnd_bollard_ContainerCreateBody = {
	Hostname: string?,
	Domainname: string?,
	User: string?,
	AttachStdin: boolean?,
	AttachStdout: boolean?,
	AttachStderr: boolean?,
	ExposedPorts: { [string]: { [string]: nil } }?,
	Tty: boolean?,
	OpenStdin: boolean?,
	StdinOnce: boolean?,
	Env: { string }?,
	Cmd: { string }?,
	Healthcheck: vnd_bollard_HealthConfig?,
	ArgsEscaped: boolean?,
	Image: string?,
	Volumes: { [string]: { [string]: nil } }?,
	WorkingDir: string?,
	Entrypoint: { string }?,
	NetworkDisabled: boolean?,
	OnBuild: { string }?,
	Labels: { [string]: string }?,
	StopSignal: string?,
	StopTimeout: number?,
	Shell: { string }?,
	HostConfig: vnd_bollard_HostConfig?,
	NetworkingConfig: vnd_bollard_NetworkingConfig?,
}

export type vnd_bollard_ContainerSummary = {
	Id: string?,
	Names: { string }?,
	Image: string?,
	ImageID: string?,
	ImageManifestDescriptor: vnd_bollard_OciDescriptor?,
	Command: string?,
	Created: number?,
	Ports: { vnd_bollard_PortSummary }?,
	SizeRw: number?,
	SizeRootFs: number?,
	Labels: { [string]: string }?,
	State: vnd_bollard_ContainerSummaryStateEnum?,
	Status: string?,
	HostConfig: vnd_bollard_ContainerSummaryHostConfig?,
	NetworkSettings: vnd_bollard_ContainerSummaryNetworkSettings?,
	Mounts: { vnd_bollard_MountPoint }?,
	Health: vnd_bollard_ContainerSummaryHealth?,
}

export type vnd_bollard_ContainerSummaryHealth = {
	Status: vnd_bollard_ContainerSummaryHealthStatusEnum?,
	FailingStreak: number?,
}

export type vnd_bollard_ContainerSummaryHealthStatusEnum = "" | "none" | "starting" | "healthy" | "unhealthy"

export type vnd_bollard_ContainerSummaryHostConfig = {
	NetworkMode: string?,
	Annotations: { [string]: string }?,
}

export type vnd_bollard_ContainerSummaryNetworkSettings = {
	Networks: { [string]: vnd_bollard_EndpointSettings }?,
}

export type vnd_bollard_ContainerSummaryStateEnum = "" | "created" | "running" | "paused" | "restarting" | "exited" | "removing" | "dead"

export type vnd_bollard_DeviceMapping = {
	PathOnHost: string?,
	PathInContainer: string?,
	CgroupPermissions: string?,
}

export type vnd_bollard_DeviceRequest = {
	Driver: string?,
	Count: number?,
	DeviceIDs: { string }?,
	Capabilities: { { string } }?,
	Options: { [string]: string }?,
}

--- The kinds of `vnd_bollard_DockerError`.
export type vnd_bollard_DockerErrorKind = "connect" | "not_found" | "conflict"

--- Why a Docker call failed.
export type vnd_bollard_DockerError =
	--- No daemon answered at `host`.
	{ kind: "connect", host: string, message: string, op: string }
	--- There is no such container.
	| { kind: "not_found", name: string, message: string, op: string }
	--- The container's state does not allow it (a name in use, a running container).
	| { kind: "conflict", reason: string, message: string, op: string }

export type vnd_bollard_EndpointIpamConfig = {
	IPv4Address: string?,
	IPv6Address: string?,
	LinkLocalIPs: { string }?,
}

export type vnd_bollard_EndpointSettings = {
	IPAMConfig: vnd_bollard_EndpointIpamConfig?,
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

export type vnd_bollard_HealthConfig = {
	Test: { string }?,
	Interval: number?,
	Timeout: number?,
	Retries: number?,
	StartPeriod: number?,
	StartInterval: number?,
}

export type vnd_bollard_HostConfig = {
	CpuShares: number?,
	Memory: number?,
	CgroupParent: string?,
	BlkioWeight: number?,
	BlkioWeightDevice: { vnd_bollard_ResourcesBlkioWeightDevice }?,
	BlkioDeviceReadBps: { vnd_bollard_ThrottleDevice }?,
	BlkioDeviceWriteBps: { vnd_bollard_ThrottleDevice }?,
	BlkioDeviceReadIOps: { vnd_bollard_ThrottleDevice }?,
	BlkioDeviceWriteIOps: { vnd_bollard_ThrottleDevice }?,
	CpuPeriod: number?,
	CpuQuota: number?,
	CpuRealtimePeriod: number?,
	CpuRealtimeRuntime: number?,
	CpusetCpus: string?,
	CpusetMems: string?,
	Devices: { vnd_bollard_DeviceMapping }?,
	DeviceCgroupRules: { string }?,
	DeviceRequests: { vnd_bollard_DeviceRequest }?,
	MemoryReservation: number?,
	MemorySwap: number?,
	MemorySwappiness: number?,
	NanoCpus: number?,
	OomKillDisable: boolean?,
	Init: boolean?,
	PidsLimit: number?,
	Ulimits: { vnd_bollard_ResourcesUlimits }?,
	CpuCount: number?,
	CpuPercent: number?,
	IOMaximumIOps: number?,
	IOMaximumBandwidth: number?,
	Binds: { string }?,
	ContainerIDFile: string?,
	LogConfig: vnd_bollard_HostConfigLogConfig?,
	NetworkMode: string?,
	PortBindings: { [string]: { vnd_bollard_PortBinding } }?,
	RestartPolicy: vnd_bollard_RestartPolicy?,
	AutoRemove: boolean?,
	VolumeDriver: string?,
	VolumesFrom: { string }?,
	Mounts: { vnd_bollard_Mount }?,
	ConsoleSize: { number }?,
	Annotations: { [string]: string }?,
	CapAdd: { string }?,
	CapDrop: { string }?,
	CgroupnsMode: vnd_bollard_HostConfigCgroupnsModeEnum?,
	Dns: { string }?,
	DnsOptions: { string }?,
	DnsSearch: { string }?,
	ExtraHosts: { string }?,
	GroupAdd: { string }?,
	IpcMode: string?,
	Cgroup: string?,
	Links: { string }?,
	OomScoreAdj: number?,
	PidMode: string?,
	Privileged: boolean?,
	PublishAllPorts: boolean?,
	ReadonlyRootfs: boolean?,
	SecurityOpt: { string }?,
	StorageOpt: { [string]: string }?,
	Tmpfs: { [string]: string }?,
	UTSMode: string?,
	UsernsMode: string?,
	ShmSize: number?,
	Sysctls: { [string]: string }?,
	Runtime: string?,
	Isolation: vnd_bollard_HostConfigIsolationEnum?,
	MaskedPaths: { string }?,
	ReadonlyPaths: { string }?,
}

export type vnd_bollard_HostConfigCgroupnsModeEnum = "" | "private" | "host"

export type vnd_bollard_HostConfigIsolationEnum = "default" | "process" | "hyperv" | ""

export type vnd_bollard_HostConfigLogConfig = {
	Type: string?,
	Config: { [string]: string }?,
}

--- Options of `list_containers`.
export type vnd_bollard_ListContainers = {
	--- Stopped containers too. Default: `false`.
	all: boolean?,
	--- At most this many.
	limit: number?,
}

--- A piece of a container's output.
export type vnd_bollard_LogPiece = {
	--- Which output it came from.
	stream: vnd_bollard_Output,
	--- What was written.
	data: string,
}

--- Options of `logs`.
export type vnd_bollard_Logs = {
	--- Start this many lines from the end.
	tail: number?,
}

export type vnd_bollard_Mount = {
	Target: string?,
	Source: string?,
	Type: vnd_bollard_MountTypeEnum?,
	ReadOnly: boolean?,
	Consistency: string?,
	BindOptions: vnd_bollard_MountBindOptions?,
	VolumeOptions: vnd_bollard_MountVolumeOptions?,
	ImageOptions: vnd_bollard_MountImageOptions?,
	TmpfsOptions: vnd_bollard_MountTmpfsOptions?,
}

export type vnd_bollard_MountBindOptions = {
	Propagation: vnd_bollard_MountBindOptionsPropagationEnum?,
	NonRecursive: boolean?,
	CreateMountpoint: boolean?,
	ReadOnlyNonRecursive: boolean?,
	ReadOnlyForceRecursive: boolean?,
}

export type vnd_bollard_MountBindOptionsPropagationEnum = "" | "private" | "rprivate" | "shared" | "rshared" | "slave" | "rslave"

export type vnd_bollard_MountImageOptions = {
	Subpath: string?,
}

export type vnd_bollard_MountPoint = {
	Type: vnd_bollard_MountPointTypeEnum?,
	Name: string?,
	Source: string?,
	Destination: string?,
	Driver: string?,
	Mode: string?,
	RW: boolean?,
	Propagation: string?,
}

export type vnd_bollard_MountPointTypeEnum = "" | "bind" | "volume" | "image" | "tmpfs" | "npipe" | "cluster"

export type vnd_bollard_MountTmpfsOptions = {
	SizeBytes: number?,
	Mode: number?,
	Options: { { string } }?,
}

export type vnd_bollard_MountTypeEnum = "" | "bind" | "volume" | "image" | "tmpfs" | "npipe" | "cluster"

export type vnd_bollard_MountVolumeOptions = {
	NoCopy: boolean?,
	Labels: { [string]: string }?,
	DriverConfig: vnd_bollard_MountVolumeOptionsDriverConfig?,
	Subpath: string?,
}

export type vnd_bollard_MountVolumeOptionsDriverConfig = {
	Name: string?,
	Options: { [string]: string }?,
}

export type vnd_bollard_NetworkingConfig = {
	EndpointsConfig: { [string]: vnd_bollard_EndpointSettings }?,
}

export type vnd_bollard_OciDescriptor = {
	mediaType: string?,
	digest: string?,
	size: number?,
	urls: { string }?,
	annotations: { [string]: string }?,
	data: string?,
	platform: vnd_bollard_OciPlatform?,
	artifactType: string?,
}

export type vnd_bollard_OciPlatform = {
	architecture: string?,
	os: string?,
	["os.version"]: string?,
	["os.features"]: { string }?,
	variant: string?,
}

--- Which output a piece came from.
export type vnd_bollard_Output = "stdout" | "stderr"

export type vnd_bollard_PortBinding = {
	HostIp: string?,
	HostPort: string?,
}

export type vnd_bollard_PortSummary = {
	IP: string?,
	PrivatePort: number,
	PublicPort: number?,
	Type: string?,
}

export type vnd_bollard_ResourcesBlkioWeightDevice = {
	Path: string?,
	Weight: number?,
}

export type vnd_bollard_ResourcesUlimits = {
	Name: string?,
	Soft: number?,
	Hard: number?,
}

export type vnd_bollard_RestartPolicy = {
	Name: vnd_bollard_RestartPolicyNameEnum?,
	MaximumRetryCount: number?,
}

export type vnd_bollard_RestartPolicyNameEnum = "" | "no" | "always" | "unless-stopped" | "on-failure"

export type vnd_bollard_ThrottleDevice = {
	Path: string?,
	Rate: number?,
}

--- The three files of a TLS client: all three or none, which the type says (not a runtime check).
export type vnd_bollard_Tls = {
	ca: string,
	cert: string,
	key: string,
}

export type vnd_bollard = {
	--- Connect to a daemon. Sends nothing: the first call does.
	---
	--- ```luau
	--- local docker = assert(bollard.connect())
	--- for _, c in assert(docker:list_containers({ all = true })) do
	---     assert(c.Names ~= nil)
	--- end
	--- ```
	connect: (opts: vnd_bollard_ConnectOptions?) -> (vnd_bollard_Client?, vnd_bollard_DockerError?),
}
