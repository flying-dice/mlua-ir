// ===== vnd:bollard =====

/** A client for one daemon. */
interface vnd_bollard_Client {
  /** The daemon this client talks to. */
  readonly host: string
  /**
   * The containers, as Docker's summaries. Only running ones unless `all`.
   *
   * Yields the calling coroutine while it waits.
   */
  list_containers(opts?: vnd_bollard_ListContainers): LuaMultiReturn<[vnd_bollard_ContainerSummary[], undefined] | [undefined, vnd_bollard_DockerError]>
  /**
   * One container's summary, or `nil` when there is none: absence is not an error.
   *
   * Yields the calling coroutine while it waits.
   */
  find_container(name: std_names_ContainerName): LuaMultiReturn<[vnd_bollard_ContainerSummary | undefined, undefined] | [undefined, vnd_bollard_DockerError]>
  /**
   * Create a container from Docker's own create body (`Image`, `Labels`, `Env`, `HostConfig`, ...): its id.
   *
   * ```luau
   * local docker = assert(bollard.connect())
   * local id = assert(docker:create_container("api", { Image = "nginx:1.27", Labels = { app = "api" } }))
   * assert(docker:find_container("api") ~= nil and #id > 0)
   * ```
   *
   * Yields the calling coroutine while it waits.
   */
  create_container(name: std_names_ContainerName, body: vnd_bollard_ContainerCreateBody): LuaMultiReturn<[string, undefined] | [undefined, vnd_bollard_DockerError]>
  /** A container's output as a stream; a container that does not exist is the stream's error. */
  logs(name: std_names_ContainerName, opts?: vnd_bollard_Logs): mlua_Stream<vnd_bollard_LogPiece, vnd_bollard_DockerError>
}

/** How to reach a daemon. */
interface vnd_bollard_ConnectOptions {
  /** `unix:///var/run/docker.sock` or `tcp://host:2375`; the local socket when not given. */
  host?: string
  /** Seconds a call waits for the daemon. */
  timeout?: number
  /** Client certificates for a TLS daemon. */
  tls?: vnd_bollard_Tls
}

interface vnd_bollard_ContainerCreateBody {
  Hostname?: string
  Domainname?: string
  User?: string
  AttachStdin?: boolean
  AttachStdout?: boolean
  AttachStderr?: boolean
  ExposedPorts?: { [key: string]: { [key: string]: undefined } }
  Tty?: boolean
  OpenStdin?: boolean
  StdinOnce?: boolean
  Env?: string[]
  Cmd?: string[]
  Healthcheck?: vnd_bollard_HealthConfig
  ArgsEscaped?: boolean
  Image?: string
  Volumes?: { [key: string]: { [key: string]: undefined } }
  WorkingDir?: string
  Entrypoint?: string[]
  NetworkDisabled?: boolean
  OnBuild?: string[]
  Labels?: { [key: string]: string }
  StopSignal?: string
  StopTimeout?: number
  Shell?: string[]
  HostConfig?: vnd_bollard_HostConfig
  NetworkingConfig?: vnd_bollard_NetworkingConfig
}

interface vnd_bollard_ContainerSummary {
  Id?: string
  Names?: string[]
  Image?: string
  ImageID?: string
  ImageManifestDescriptor?: vnd_bollard_OciDescriptor
  Command?: string
  Created?: number
  Ports?: vnd_bollard_PortSummary[]
  SizeRw?: number
  SizeRootFs?: number
  Labels?: { [key: string]: string }
  State?: vnd_bollard_ContainerSummaryStateEnum
  Status?: string
  HostConfig?: vnd_bollard_ContainerSummaryHostConfig
  NetworkSettings?: vnd_bollard_ContainerSummaryNetworkSettings
  Mounts?: vnd_bollard_MountPoint[]
  Health?: vnd_bollard_ContainerSummaryHealth
}

interface vnd_bollard_ContainerSummaryHealth {
  Status?: vnd_bollard_ContainerSummaryHealthStatusEnum
  FailingStreak?: number
}

type vnd_bollard_ContainerSummaryHealthStatusEnum = "" | "none" | "starting" | "healthy" | "unhealthy"

interface vnd_bollard_ContainerSummaryHostConfig {
  NetworkMode?: string
  Annotations?: { [key: string]: string }
}

interface vnd_bollard_ContainerSummaryNetworkSettings {
  Networks?: { [key: string]: vnd_bollard_EndpointSettings }
}

type vnd_bollard_ContainerSummaryStateEnum = "" | "created" | "running" | "paused" | "restarting" | "exited" | "removing" | "dead"

interface vnd_bollard_DeviceMapping {
  PathOnHost?: string
  PathInContainer?: string
  CgroupPermissions?: string
}

interface vnd_bollard_DeviceRequest {
  Driver?: string
  Count?: number
  DeviceIDs?: string[]
  Capabilities?: string[][]
  Options?: { [key: string]: string }
}

/** The kinds of `vnd_bollard_DockerError`. */
type vnd_bollard_DockerErrorKind = "connect" | "not_found" | "conflict"

/** Why a Docker call failed. */
type vnd_bollard_DockerError =
  /** No daemon answered at `host`. */
  | { kind: "connect"; host: string; message: string; op: string }
  /** There is no such container. */
  | { kind: "not_found"; name: string; message: string; op: string }
  /** The container's state does not allow it (a name in use, a running container). */
  | { kind: "conflict"; reason: string; message: string; op: string }

interface vnd_bollard_EndpointIpamConfig {
  IPv4Address?: string
  IPv6Address?: string
  LinkLocalIPs?: string[]
}

interface vnd_bollard_EndpointSettings {
  IPAMConfig?: vnd_bollard_EndpointIpamConfig
  Links?: string[]
  MacAddress?: string
  Aliases?: string[]
  DriverOpts?: { [key: string]: string }
  GwPriority?: number
  NetworkID?: string
  EndpointID?: string
  Gateway?: string
  IPAddress?: string
  IPPrefixLen?: number
  IPv6Gateway?: string
  GlobalIPv6Address?: string
  GlobalIPv6PrefixLen?: number
  DNSNames?: string[]
}

interface vnd_bollard_HealthConfig {
  Test?: string[]
  Interval?: number
  Timeout?: number
  Retries?: number
  StartPeriod?: number
  StartInterval?: number
}

interface vnd_bollard_HostConfig {
  CpuShares?: number
  Memory?: number
  CgroupParent?: string
  BlkioWeight?: number
  BlkioWeightDevice?: vnd_bollard_ResourcesBlkioWeightDevice[]
  BlkioDeviceReadBps?: vnd_bollard_ThrottleDevice[]
  BlkioDeviceWriteBps?: vnd_bollard_ThrottleDevice[]
  BlkioDeviceReadIOps?: vnd_bollard_ThrottleDevice[]
  BlkioDeviceWriteIOps?: vnd_bollard_ThrottleDevice[]
  CpuPeriod?: number
  CpuQuota?: number
  CpuRealtimePeriod?: number
  CpuRealtimeRuntime?: number
  CpusetCpus?: string
  CpusetMems?: string
  Devices?: vnd_bollard_DeviceMapping[]
  DeviceCgroupRules?: string[]
  DeviceRequests?: vnd_bollard_DeviceRequest[]
  MemoryReservation?: number
  MemorySwap?: number
  MemorySwappiness?: number
  NanoCpus?: number
  OomKillDisable?: boolean
  Init?: boolean
  PidsLimit?: number
  Ulimits?: vnd_bollard_ResourcesUlimits[]
  CpuCount?: number
  CpuPercent?: number
  IOMaximumIOps?: number
  IOMaximumBandwidth?: number
  Binds?: string[]
  ContainerIDFile?: string
  LogConfig?: vnd_bollard_HostConfigLogConfig
  NetworkMode?: string
  PortBindings?: { [key: string]: vnd_bollard_PortBinding[] | undefined }
  RestartPolicy?: vnd_bollard_RestartPolicy
  AutoRemove?: boolean
  VolumeDriver?: string
  VolumesFrom?: string[]
  Mounts?: vnd_bollard_Mount[]
  ConsoleSize?: number[]
  Annotations?: { [key: string]: string }
  CapAdd?: string[]
  CapDrop?: string[]
  CgroupnsMode?: vnd_bollard_HostConfigCgroupnsModeEnum
  Dns?: string[]
  DnsOptions?: string[]
  DnsSearch?: string[]
  ExtraHosts?: string[]
  GroupAdd?: string[]
  IpcMode?: string
  Cgroup?: string
  Links?: string[]
  OomScoreAdj?: number
  PidMode?: string
  Privileged?: boolean
  PublishAllPorts?: boolean
  ReadonlyRootfs?: boolean
  SecurityOpt?: string[]
  StorageOpt?: { [key: string]: string }
  Tmpfs?: { [key: string]: string }
  UTSMode?: string
  UsernsMode?: string
  ShmSize?: number
  Sysctls?: { [key: string]: string }
  Runtime?: string
  Isolation?: vnd_bollard_HostConfigIsolationEnum
  MaskedPaths?: string[]
  ReadonlyPaths?: string[]
}

type vnd_bollard_HostConfigCgroupnsModeEnum = "" | "private" | "host"

type vnd_bollard_HostConfigIsolationEnum = "default" | "process" | "hyperv" | ""

interface vnd_bollard_HostConfigLogConfig {
  Type?: string
  Config?: { [key: string]: string }
}

/** Options of `list_containers`. */
interface vnd_bollard_ListContainers {
  /** Stopped containers too. @default false */
  all?: boolean
  /** At most this many. */
  limit?: number
}

/** A piece of a container's output. */
interface vnd_bollard_LogPiece {
  /** Which output it came from. */
  stream: vnd_bollard_Output
  /** What was written. */
  data: string
}

/** Options of `logs`. */
interface vnd_bollard_Logs {
  /** Start this many lines from the end. */
  tail?: number
}

interface vnd_bollard_Mount {
  Target?: string
  Source?: string
  Type?: vnd_bollard_MountTypeEnum
  ReadOnly?: boolean
  Consistency?: string
  BindOptions?: vnd_bollard_MountBindOptions
  VolumeOptions?: vnd_bollard_MountVolumeOptions
  ImageOptions?: vnd_bollard_MountImageOptions
  TmpfsOptions?: vnd_bollard_MountTmpfsOptions
}

interface vnd_bollard_MountBindOptions {
  Propagation?: vnd_bollard_MountBindOptionsPropagationEnum
  NonRecursive?: boolean
  CreateMountpoint?: boolean
  ReadOnlyNonRecursive?: boolean
  ReadOnlyForceRecursive?: boolean
}

type vnd_bollard_MountBindOptionsPropagationEnum = "" | "private" | "rprivate" | "shared" | "rshared" | "slave" | "rslave"

interface vnd_bollard_MountImageOptions {
  Subpath?: string
}

interface vnd_bollard_MountPoint {
  Type?: vnd_bollard_MountPointTypeEnum
  Name?: string
  Source?: string
  Destination?: string
  Driver?: string
  Mode?: string
  RW?: boolean
  Propagation?: string
}

type vnd_bollard_MountPointTypeEnum = "" | "bind" | "volume" | "image" | "tmpfs" | "npipe" | "cluster"

interface vnd_bollard_MountTmpfsOptions {
  SizeBytes?: number
  Mode?: number
  Options?: string[][]
}

type vnd_bollard_MountTypeEnum = "" | "bind" | "volume" | "image" | "tmpfs" | "npipe" | "cluster"

interface vnd_bollard_MountVolumeOptions {
  NoCopy?: boolean
  Labels?: { [key: string]: string }
  DriverConfig?: vnd_bollard_MountVolumeOptionsDriverConfig
  Subpath?: string
}

interface vnd_bollard_MountVolumeOptionsDriverConfig {
  Name?: string
  Options?: { [key: string]: string }
}

interface vnd_bollard_NetworkingConfig {
  EndpointsConfig?: { [key: string]: vnd_bollard_EndpointSettings }
}

interface vnd_bollard_OciDescriptor {
  mediaType?: string
  digest?: string
  size?: number
  urls?: string[]
  annotations?: { [key: string]: string }
  data?: string
  platform?: vnd_bollard_OciPlatform
  artifactType?: string
}

interface vnd_bollard_OciPlatform {
  architecture?: string
  os?: string
  "os.version"?: string
  "os.features"?: string[]
  variant?: string
}

/** Which output a piece came from. */
type vnd_bollard_Output = "stdout" | "stderr"

interface vnd_bollard_PortBinding {
  HostIp?: string
  HostPort?: string
}

interface vnd_bollard_PortSummary {
  IP?: string
  PrivatePort: number
  PublicPort?: number
  Type?: string
}

interface vnd_bollard_ResourcesBlkioWeightDevice {
  Path?: string
  Weight?: number
}

interface vnd_bollard_ResourcesUlimits {
  Name?: string
  Soft?: number
  Hard?: number
}

interface vnd_bollard_RestartPolicy {
  Name?: vnd_bollard_RestartPolicyNameEnum
  MaximumRetryCount?: number
}

type vnd_bollard_RestartPolicyNameEnum = "" | "no" | "always" | "unless-stopped" | "on-failure"

interface vnd_bollard_ThrottleDevice {
  Path?: string
  Rate?: number
}

/** The three files of a TLS client: all three or none, which the type says (not a runtime check). */
interface vnd_bollard_Tls {
  ca: string
  cert: string
  key: string
}

declare module "vnd:bollard" {
  /**
   * Connect to a daemon. Sends nothing: the first call does.
   *
   * ```luau
   * local docker = assert(bollard.connect())
   * for _, c in assert(docker:list_containers({ all = true })) do
   *     assert(c.Names ~= nil)
   * end
   * ```
   */
  export function connect(this: void, opts?: vnd_bollard_ConnectOptions): LuaMultiReturn<[vnd_bollard_Client, undefined] | [undefined, vnd_bollard_DockerError]>
  export type Client = vnd_bollard_Client
  export type ConnectOptions = vnd_bollard_ConnectOptions
  export type ContainerCreateBody = vnd_bollard_ContainerCreateBody
  export type ContainerSummary = vnd_bollard_ContainerSummary
  export type ContainerSummaryHealth = vnd_bollard_ContainerSummaryHealth
  export type ContainerSummaryHealthStatusEnum = vnd_bollard_ContainerSummaryHealthStatusEnum
  export type ContainerSummaryHostConfig = vnd_bollard_ContainerSummaryHostConfig
  export type ContainerSummaryNetworkSettings = vnd_bollard_ContainerSummaryNetworkSettings
  export type ContainerSummaryStateEnum = vnd_bollard_ContainerSummaryStateEnum
  export type DeviceMapping = vnd_bollard_DeviceMapping
  export type DeviceRequest = vnd_bollard_DeviceRequest
  export type DockerError = vnd_bollard_DockerError
  export type EndpointIpamConfig = vnd_bollard_EndpointIpamConfig
  export type EndpointSettings = vnd_bollard_EndpointSettings
  export type HealthConfig = vnd_bollard_HealthConfig
  export type HostConfig = vnd_bollard_HostConfig
  export type HostConfigCgroupnsModeEnum = vnd_bollard_HostConfigCgroupnsModeEnum
  export type HostConfigIsolationEnum = vnd_bollard_HostConfigIsolationEnum
  export type HostConfigLogConfig = vnd_bollard_HostConfigLogConfig
  export type ListContainers = vnd_bollard_ListContainers
  export type LogPiece = vnd_bollard_LogPiece
  export type Logs = vnd_bollard_Logs
  export type Mount = vnd_bollard_Mount
  export type MountBindOptions = vnd_bollard_MountBindOptions
  export type MountBindOptionsPropagationEnum = vnd_bollard_MountBindOptionsPropagationEnum
  export type MountImageOptions = vnd_bollard_MountImageOptions
  export type MountPoint = vnd_bollard_MountPoint
  export type MountPointTypeEnum = vnd_bollard_MountPointTypeEnum
  export type MountTmpfsOptions = vnd_bollard_MountTmpfsOptions
  export type MountTypeEnum = vnd_bollard_MountTypeEnum
  export type MountVolumeOptions = vnd_bollard_MountVolumeOptions
  export type MountVolumeOptionsDriverConfig = vnd_bollard_MountVolumeOptionsDriverConfig
  export type NetworkingConfig = vnd_bollard_NetworkingConfig
  export type OciDescriptor = vnd_bollard_OciDescriptor
  export type OciPlatform = vnd_bollard_OciPlatform
  export type Output = vnd_bollard_Output
  export type PortBinding = vnd_bollard_PortBinding
  export type PortSummary = vnd_bollard_PortSummary
  export type ResourcesBlkioWeightDevice = vnd_bollard_ResourcesBlkioWeightDevice
  export type ResourcesUlimits = vnd_bollard_ResourcesUlimits
  export type RestartPolicy = vnd_bollard_RestartPolicy
  export type RestartPolicyNameEnum = vnd_bollard_RestartPolicyNameEnum
  export type ThrottleDevice = vnd_bollard_ThrottleDevice
  export type Tls = vnd_bollard_Tls
}
