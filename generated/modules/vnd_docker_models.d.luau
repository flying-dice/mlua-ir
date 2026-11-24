-- ===== vnd:docker_models =====

export type vnd_docker_models_ContainerConfig = {
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
	Healthcheck: vnd_docker_models_HealthConfig?,
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
}

export type vnd_docker_models_ContainerInspectResponse = {
	Id: string?,
	Created: string?,
	Path: string?,
	Args: { string }?,
	State: vnd_docker_models_ContainerState?,
	Image: string?,
	ResolvConfPath: string?,
	HostnamePath: string?,
	HostsPath: string?,
	LogPath: string?,
	Name: string?,
	RestartCount: number?,
	Driver: string?,
	Platform: string?,
	ImageManifestDescriptor: vnd_docker_models_OciDescriptor?,
	MountLabel: string?,
	ProcessLabel: string?,
	AppArmorProfile: string?,
	ExecIDs: { string }?,
	HostConfig: vnd_docker_models_HostConfig?,
	GraphDriver: vnd_docker_models_DriverData?,
	Storage: vnd_docker_models_Storage?,
	SizeRw: number?,
	SizeRootFs: number?,
	Mounts: { vnd_docker_models_MountPoint }?,
	Config: vnd_docker_models_ContainerConfig?,
	NetworkSettings: vnd_docker_models_NetworkSettings?,
}

export type vnd_docker_models_ContainerState = {
	Status: vnd_docker_models_ContainerStateStatusEnum?,
	Running: boolean?,
	Paused: boolean?,
	Restarting: boolean?,
	OOMKilled: boolean?,
	Dead: boolean?,
	Pid: number?,
	ExitCode: number?,
	Error: string?,
	StartedAt: string?,
	FinishedAt: string?,
	Health: vnd_docker_models_Health?,
}

export type vnd_docker_models_ContainerStateStatusEnum = "" | "created" | "running" | "paused" | "restarting" | "removing" | "exited" | "dead"

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

export type vnd_docker_models_DeviceMapping = {
	PathOnHost: string?,
	PathInContainer: string?,
	CgroupPermissions: string?,
}

export type vnd_docker_models_DeviceRequest = {
	Driver: string?,
	Count: number?,
	DeviceIDs: { string }?,
	Capabilities: { { string } }?,
	Options: { [string]: string }?,
}

export type vnd_docker_models_DriverData = {
	Name: string,
	Data: { [string]: string }?,
}

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

export type vnd_docker_models_Health = {
	Status: vnd_docker_models_HealthStatusEnum?,
	FailingStreak: number?,
	Log: { vnd_docker_models_HealthcheckResult }?,
}

export type vnd_docker_models_HealthConfig = {
	Test: { string }?,
	Interval: number?,
	Timeout: number?,
	Retries: number?,
	StartPeriod: number?,
	StartInterval: number?,
}

export type vnd_docker_models_HealthStatusEnum = "" | "none" | "starting" | "healthy" | "unhealthy"

export type vnd_docker_models_HealthcheckResult = {
	Start: string?,
	End: string?,
	ExitCode: number?,
	Output: string?,
}

export type vnd_docker_models_HostConfig = {
	CpuShares: number?,
	Memory: number?,
	CgroupParent: string?,
	BlkioWeight: number?,
	BlkioWeightDevice: { vnd_docker_models_ResourcesBlkioWeightDevice }?,
	BlkioDeviceReadBps: { vnd_docker_models_ThrottleDevice }?,
	BlkioDeviceWriteBps: { vnd_docker_models_ThrottleDevice }?,
	BlkioDeviceReadIOps: { vnd_docker_models_ThrottleDevice }?,
	BlkioDeviceWriteIOps: { vnd_docker_models_ThrottleDevice }?,
	CpuPeriod: number?,
	CpuQuota: number?,
	CpuRealtimePeriod: number?,
	CpuRealtimeRuntime: number?,
	CpusetCpus: string?,
	CpusetMems: string?,
	Devices: { vnd_docker_models_DeviceMapping }?,
	DeviceCgroupRules: { string }?,
	DeviceRequests: { vnd_docker_models_DeviceRequest }?,
	MemoryReservation: number?,
	MemorySwap: number?,
	MemorySwappiness: number?,
	NanoCpus: number?,
	OomKillDisable: boolean?,
	Init: boolean?,
	PidsLimit: number?,
	Ulimits: { vnd_docker_models_ResourcesUlimits }?,
	CpuCount: number?,
	CpuPercent: number?,
	IOMaximumIOps: number?,
	IOMaximumBandwidth: number?,
	Binds: { string }?,
	ContainerIDFile: string?,
	LogConfig: vnd_docker_models_HostConfigLogConfig?,
	NetworkMode: string?,
	PortBindings: { [string]: { vnd_docker_models_PortBinding } }?,
	RestartPolicy: vnd_docker_models_RestartPolicy?,
	AutoRemove: boolean?,
	VolumeDriver: string?,
	VolumesFrom: { string }?,
	Mounts: { vnd_docker_models_Mount }?,
	ConsoleSize: { number }?,
	Annotations: { [string]: string }?,
	CapAdd: { string }?,
	CapDrop: { string }?,
	CgroupnsMode: vnd_docker_models_HostConfigCgroupnsModeEnum?,
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
	Isolation: vnd_docker_models_HostConfigIsolationEnum?,
	MaskedPaths: { string }?,
	ReadonlyPaths: { string }?,
}

export type vnd_docker_models_HostConfigCgroupnsModeEnum = "" | "private" | "host"

export type vnd_docker_models_HostConfigIsolationEnum = "default" | "process" | "hyperv" | ""

export type vnd_docker_models_HostConfigLogConfig = {
	Type: string?,
	Config: { [string]: string }?,
}

export type vnd_docker_models_Mount = {
	Target: string?,
	Source: string?,
	Type: vnd_docker_models_MountTypeEnum?,
	ReadOnly: boolean?,
	Consistency: string?,
	BindOptions: vnd_docker_models_MountBindOptions?,
	VolumeOptions: vnd_docker_models_MountVolumeOptions?,
	ImageOptions: vnd_docker_models_MountImageOptions?,
	TmpfsOptions: vnd_docker_models_MountTmpfsOptions?,
}

export type vnd_docker_models_MountBindOptions = {
	Propagation: vnd_docker_models_MountBindOptionsPropagationEnum?,
	NonRecursive: boolean?,
	CreateMountpoint: boolean?,
	ReadOnlyNonRecursive: boolean?,
	ReadOnlyForceRecursive: boolean?,
}

export type vnd_docker_models_MountBindOptionsPropagationEnum = "" | "private" | "rprivate" | "shared" | "rshared" | "slave" | "rslave"

export type vnd_docker_models_MountImageOptions = {
	Subpath: string?,
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

export type vnd_docker_models_MountTmpfsOptions = {
	SizeBytes: number?,
	Mode: number?,
	Options: { { string } }?,
}

export type vnd_docker_models_MountTypeEnum = "" | "bind" | "volume" | "image" | "tmpfs" | "npipe" | "cluster"

export type vnd_docker_models_MountVolumeOptions = {
	NoCopy: boolean?,
	Labels: { [string]: string }?,
	DriverConfig: vnd_docker_models_MountVolumeOptionsDriverConfig?,
	Subpath: string?,
}

export type vnd_docker_models_MountVolumeOptionsDriverConfig = {
	Name: string?,
	Options: { [string]: string }?,
}

export type vnd_docker_models_NetworkSettings = {
	SandboxID: string?,
	SandboxKey: string?,
	Ports: { [string]: { vnd_docker_models_PortBinding } }?,
	Networks: { [string]: vnd_docker_models_EndpointSettings }?,
}

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

export type vnd_docker_models_PortBinding = {
	HostIp: string?,
	HostPort: string?,
}

export type vnd_docker_models_PortSummary = {
	IP: string?,
	PrivatePort: number,
	PublicPort: number?,
	Type: string?,
}

export type vnd_docker_models_ResourcesBlkioWeightDevice = {
	Path: string?,
	Weight: number?,
}

export type vnd_docker_models_ResourcesUlimits = {
	Name: string?,
	Soft: number?,
	Hard: number?,
}

export type vnd_docker_models_RestartPolicy = {
	Name: vnd_docker_models_RestartPolicyNameEnum?,
	MaximumRetryCount: number?,
}

export type vnd_docker_models_RestartPolicyNameEnum = "" | "no" | "always" | "unless-stopped" | "on-failure"

export type vnd_docker_models_RootFsStorage = {
	Snapshot: vnd_docker_models_RootFsStorageSnapshot?,
}

export type vnd_docker_models_RootFsStorageSnapshot = {
	Name: string?,
}

export type vnd_docker_models_Storage = {
	RootFS: vnd_docker_models_RootFsStorage?,
}

export type vnd_docker_models_ThrottleDevice = {
	Path: string?,
	Rate: number?,
}

export type vnd_docker_models = {
	--- Two containers as Docker reports them: bollard's own `ContainerSummary`, typed field by field.
	---
	--- ```luau
	--- local containers = docker_models.sample()
	--- assert(containers[1].State == "running")
	--- ```
	sample: () -> { vnd_docker_models_ContainerSummary },
	--- A container's inspect answer as bollard models it: its dates are parsed text (bollard's `time` feature), which
	--- tracing reads through a valid date-time sample.
	inspect_sample: () -> vnd_docker_models_ContainerInspectResponse,
	--- One line for a container: its first name and its state. Takes bollard's type as an argument too.
	line: (container: vnd_docker_models_ContainerSummary) -> string,
}
