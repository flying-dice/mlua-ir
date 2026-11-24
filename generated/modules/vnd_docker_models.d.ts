// ===== vnd:docker_models =====

interface vnd_docker_models_ContainerConfig {
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
  Healthcheck?: vnd_docker_models_HealthConfig
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
}

interface vnd_docker_models_ContainerInspectResponse {
  Id?: string
  Created?: string
  Path?: string
  Args?: string[]
  State?: vnd_docker_models_ContainerState
  Image?: string
  ResolvConfPath?: string
  HostnamePath?: string
  HostsPath?: string
  LogPath?: string
  Name?: string
  RestartCount?: number
  Driver?: string
  Platform?: string
  ImageManifestDescriptor?: vnd_docker_models_OciDescriptor
  MountLabel?: string
  ProcessLabel?: string
  AppArmorProfile?: string
  ExecIDs?: string[]
  HostConfig?: vnd_docker_models_HostConfig
  GraphDriver?: vnd_docker_models_DriverData
  Storage?: vnd_docker_models_Storage
  SizeRw?: number
  SizeRootFs?: number
  Mounts?: vnd_docker_models_MountPoint[]
  Config?: vnd_docker_models_ContainerConfig
  NetworkSettings?: vnd_docker_models_NetworkSettings
}

interface vnd_docker_models_ContainerState {
  Status?: vnd_docker_models_ContainerStateStatusEnum
  Running?: boolean
  Paused?: boolean
  Restarting?: boolean
  OOMKilled?: boolean
  Dead?: boolean
  Pid?: number
  ExitCode?: number
  Error?: string
  StartedAt?: string
  FinishedAt?: string
  Health?: vnd_docker_models_Health
}

type vnd_docker_models_ContainerStateStatusEnum = "" | "created" | "running" | "paused" | "restarting" | "removing" | "exited" | "dead"

interface vnd_docker_models_ContainerSummary {
  Id?: string
  Names?: string[]
  Image?: string
  ImageID?: string
  ImageManifestDescriptor?: vnd_docker_models_OciDescriptor
  Command?: string
  Created?: number
  Ports?: vnd_docker_models_PortSummary[]
  SizeRw?: number
  SizeRootFs?: number
  Labels?: { [key: string]: string }
  State?: vnd_docker_models_ContainerSummaryStateEnum
  Status?: string
  HostConfig?: vnd_docker_models_ContainerSummaryHostConfig
  NetworkSettings?: vnd_docker_models_ContainerSummaryNetworkSettings
  Mounts?: vnd_docker_models_MountPoint[]
  Health?: vnd_docker_models_ContainerSummaryHealth
}

interface vnd_docker_models_ContainerSummaryHealth {
  Status?: vnd_docker_models_ContainerSummaryHealthStatusEnum
  FailingStreak?: number
}

type vnd_docker_models_ContainerSummaryHealthStatusEnum = "" | "none" | "starting" | "healthy" | "unhealthy"

interface vnd_docker_models_ContainerSummaryHostConfig {
  NetworkMode?: string
  Annotations?: { [key: string]: string }
}

interface vnd_docker_models_ContainerSummaryNetworkSettings {
  Networks?: { [key: string]: vnd_docker_models_EndpointSettings }
}

type vnd_docker_models_ContainerSummaryStateEnum = "" | "created" | "running" | "paused" | "restarting" | "exited" | "removing" | "dead"

interface vnd_docker_models_DeviceMapping {
  PathOnHost?: string
  PathInContainer?: string
  CgroupPermissions?: string
}

interface vnd_docker_models_DeviceRequest {
  Driver?: string
  Count?: number
  DeviceIDs?: string[]
  Capabilities?: string[][]
  Options?: { [key: string]: string }
}

interface vnd_docker_models_DriverData {
  Name: string
  Data?: { [key: string]: string }
}

interface vnd_docker_models_EndpointIpamConfig {
  IPv4Address?: string
  IPv6Address?: string
  LinkLocalIPs?: string[]
}

interface vnd_docker_models_EndpointSettings {
  IPAMConfig?: vnd_docker_models_EndpointIpamConfig
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

interface vnd_docker_models_Health {
  Status?: vnd_docker_models_HealthStatusEnum
  FailingStreak?: number
  Log?: vnd_docker_models_HealthcheckResult[]
}

interface vnd_docker_models_HealthConfig {
  Test?: string[]
  Interval?: number
  Timeout?: number
  Retries?: number
  StartPeriod?: number
  StartInterval?: number
}

type vnd_docker_models_HealthStatusEnum = "" | "none" | "starting" | "healthy" | "unhealthy"

interface vnd_docker_models_HealthcheckResult {
  Start?: string
  End?: string
  ExitCode?: number
  Output?: string
}

interface vnd_docker_models_HostConfig {
  CpuShares?: number
  Memory?: number
  CgroupParent?: string
  BlkioWeight?: number
  BlkioWeightDevice?: vnd_docker_models_ResourcesBlkioWeightDevice[]
  BlkioDeviceReadBps?: vnd_docker_models_ThrottleDevice[]
  BlkioDeviceWriteBps?: vnd_docker_models_ThrottleDevice[]
  BlkioDeviceReadIOps?: vnd_docker_models_ThrottleDevice[]
  BlkioDeviceWriteIOps?: vnd_docker_models_ThrottleDevice[]
  CpuPeriod?: number
  CpuQuota?: number
  CpuRealtimePeriod?: number
  CpuRealtimeRuntime?: number
  CpusetCpus?: string
  CpusetMems?: string
  Devices?: vnd_docker_models_DeviceMapping[]
  DeviceCgroupRules?: string[]
  DeviceRequests?: vnd_docker_models_DeviceRequest[]
  MemoryReservation?: number
  MemorySwap?: number
  MemorySwappiness?: number
  NanoCpus?: number
  OomKillDisable?: boolean
  Init?: boolean
  PidsLimit?: number
  Ulimits?: vnd_docker_models_ResourcesUlimits[]
  CpuCount?: number
  CpuPercent?: number
  IOMaximumIOps?: number
  IOMaximumBandwidth?: number
  Binds?: string[]
  ContainerIDFile?: string
  LogConfig?: vnd_docker_models_HostConfigLogConfig
  NetworkMode?: string
  PortBindings?: { [key: string]: vnd_docker_models_PortBinding[] | undefined }
  RestartPolicy?: vnd_docker_models_RestartPolicy
  AutoRemove?: boolean
  VolumeDriver?: string
  VolumesFrom?: string[]
  Mounts?: vnd_docker_models_Mount[]
  ConsoleSize?: number[]
  Annotations?: { [key: string]: string }
  CapAdd?: string[]
  CapDrop?: string[]
  CgroupnsMode?: vnd_docker_models_HostConfigCgroupnsModeEnum
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
  Isolation?: vnd_docker_models_HostConfigIsolationEnum
  MaskedPaths?: string[]
  ReadonlyPaths?: string[]
}

type vnd_docker_models_HostConfigCgroupnsModeEnum = "" | "private" | "host"

type vnd_docker_models_HostConfigIsolationEnum = "default" | "process" | "hyperv" | ""

interface vnd_docker_models_HostConfigLogConfig {
  Type?: string
  Config?: { [key: string]: string }
}

interface vnd_docker_models_Mount {
  Target?: string
  Source?: string
  Type?: vnd_docker_models_MountTypeEnum
  ReadOnly?: boolean
  Consistency?: string
  BindOptions?: vnd_docker_models_MountBindOptions
  VolumeOptions?: vnd_docker_models_MountVolumeOptions
  ImageOptions?: vnd_docker_models_MountImageOptions
  TmpfsOptions?: vnd_docker_models_MountTmpfsOptions
}

interface vnd_docker_models_MountBindOptions {
  Propagation?: vnd_docker_models_MountBindOptionsPropagationEnum
  NonRecursive?: boolean
  CreateMountpoint?: boolean
  ReadOnlyNonRecursive?: boolean
  ReadOnlyForceRecursive?: boolean
}

type vnd_docker_models_MountBindOptionsPropagationEnum = "" | "private" | "rprivate" | "shared" | "rshared" | "slave" | "rslave"

interface vnd_docker_models_MountImageOptions {
  Subpath?: string
}

interface vnd_docker_models_MountPoint {
  Type?: vnd_docker_models_MountPointTypeEnum
  Name?: string
  Source?: string
  Destination?: string
  Driver?: string
  Mode?: string
  RW?: boolean
  Propagation?: string
}

type vnd_docker_models_MountPointTypeEnum = "" | "bind" | "volume" | "image" | "tmpfs" | "npipe" | "cluster"

interface vnd_docker_models_MountTmpfsOptions {
  SizeBytes?: number
  Mode?: number
  Options?: string[][]
}

type vnd_docker_models_MountTypeEnum = "" | "bind" | "volume" | "image" | "tmpfs" | "npipe" | "cluster"

interface vnd_docker_models_MountVolumeOptions {
  NoCopy?: boolean
  Labels?: { [key: string]: string }
  DriverConfig?: vnd_docker_models_MountVolumeOptionsDriverConfig
  Subpath?: string
}

interface vnd_docker_models_MountVolumeOptionsDriverConfig {
  Name?: string
  Options?: { [key: string]: string }
}

interface vnd_docker_models_NetworkSettings {
  SandboxID?: string
  SandboxKey?: string
  Ports?: { [key: string]: vnd_docker_models_PortBinding[] | undefined }
  Networks?: { [key: string]: vnd_docker_models_EndpointSettings }
}

interface vnd_docker_models_OciDescriptor {
  mediaType?: string
  digest?: string
  size?: number
  urls?: string[]
  annotations?: { [key: string]: string }
  data?: string
  platform?: vnd_docker_models_OciPlatform
  artifactType?: string
}

interface vnd_docker_models_OciPlatform {
  architecture?: string
  os?: string
  "os.version"?: string
  "os.features"?: string[]
  variant?: string
}

interface vnd_docker_models_PortBinding {
  HostIp?: string
  HostPort?: string
}

interface vnd_docker_models_PortSummary {
  IP?: string
  PrivatePort: number
  PublicPort?: number
  Type?: string
}

interface vnd_docker_models_ResourcesBlkioWeightDevice {
  Path?: string
  Weight?: number
}

interface vnd_docker_models_ResourcesUlimits {
  Name?: string
  Soft?: number
  Hard?: number
}

interface vnd_docker_models_RestartPolicy {
  Name?: vnd_docker_models_RestartPolicyNameEnum
  MaximumRetryCount?: number
}

type vnd_docker_models_RestartPolicyNameEnum = "" | "no" | "always" | "unless-stopped" | "on-failure"

interface vnd_docker_models_RootFsStorage {
  Snapshot?: vnd_docker_models_RootFsStorageSnapshot
}

interface vnd_docker_models_RootFsStorageSnapshot {
  Name?: string
}

interface vnd_docker_models_Storage {
  RootFS?: vnd_docker_models_RootFsStorage
}

interface vnd_docker_models_ThrottleDevice {
  Path?: string
  Rate?: number
}

declare module "vnd:docker_models" {
  /**
   * Two containers as Docker reports them: bollard's own `ContainerSummary`, typed field by field.
   *
   * ```luau
   * local containers = docker_models.sample()
   * assert(containers[1].State == "running")
   * ```
   */
  export function sample(this: void): vnd_docker_models_ContainerSummary[]
  /**
   * A container's inspect answer as bollard models it: its dates are parsed text (bollard's `time` feature), which
   * tracing reads through a valid date-time sample.
   */
  export function inspect_sample(this: void): vnd_docker_models_ContainerInspectResponse
  /** One line for a container: its first name and its state. Takes bollard's type as an argument too. */
  export function line(this: void, container: vnd_docker_models_ContainerSummary): string
  export type ContainerConfig = vnd_docker_models_ContainerConfig
  export type ContainerInspectResponse = vnd_docker_models_ContainerInspectResponse
  export type ContainerState = vnd_docker_models_ContainerState
  export type ContainerStateStatusEnum = vnd_docker_models_ContainerStateStatusEnum
  export type ContainerSummary = vnd_docker_models_ContainerSummary
  export type ContainerSummaryHealth = vnd_docker_models_ContainerSummaryHealth
  export type ContainerSummaryHealthStatusEnum = vnd_docker_models_ContainerSummaryHealthStatusEnum
  export type ContainerSummaryHostConfig = vnd_docker_models_ContainerSummaryHostConfig
  export type ContainerSummaryNetworkSettings = vnd_docker_models_ContainerSummaryNetworkSettings
  export type ContainerSummaryStateEnum = vnd_docker_models_ContainerSummaryStateEnum
  export type DeviceMapping = vnd_docker_models_DeviceMapping
  export type DeviceRequest = vnd_docker_models_DeviceRequest
  export type DriverData = vnd_docker_models_DriverData
  export type EndpointIpamConfig = vnd_docker_models_EndpointIpamConfig
  export type EndpointSettings = vnd_docker_models_EndpointSettings
  export type Health = vnd_docker_models_Health
  export type HealthConfig = vnd_docker_models_HealthConfig
  export type HealthStatusEnum = vnd_docker_models_HealthStatusEnum
  export type HealthcheckResult = vnd_docker_models_HealthcheckResult
  export type HostConfig = vnd_docker_models_HostConfig
  export type HostConfigCgroupnsModeEnum = vnd_docker_models_HostConfigCgroupnsModeEnum
  export type HostConfigIsolationEnum = vnd_docker_models_HostConfigIsolationEnum
  export type HostConfigLogConfig = vnd_docker_models_HostConfigLogConfig
  export type Mount = vnd_docker_models_Mount
  export type MountBindOptions = vnd_docker_models_MountBindOptions
  export type MountBindOptionsPropagationEnum = vnd_docker_models_MountBindOptionsPropagationEnum
  export type MountImageOptions = vnd_docker_models_MountImageOptions
  export type MountPoint = vnd_docker_models_MountPoint
  export type MountPointTypeEnum = vnd_docker_models_MountPointTypeEnum
  export type MountTmpfsOptions = vnd_docker_models_MountTmpfsOptions
  export type MountTypeEnum = vnd_docker_models_MountTypeEnum
  export type MountVolumeOptions = vnd_docker_models_MountVolumeOptions
  export type MountVolumeOptionsDriverConfig = vnd_docker_models_MountVolumeOptionsDriverConfig
  export type NetworkSettings = vnd_docker_models_NetworkSettings
  export type OciDescriptor = vnd_docker_models_OciDescriptor
  export type OciPlatform = vnd_docker_models_OciPlatform
  export type PortBinding = vnd_docker_models_PortBinding
  export type PortSummary = vnd_docker_models_PortSummary
  export type ResourcesBlkioWeightDevice = vnd_docker_models_ResourcesBlkioWeightDevice
  export type ResourcesUlimits = vnd_docker_models_ResourcesUlimits
  export type RestartPolicy = vnd_docker_models_RestartPolicy
  export type RestartPolicyNameEnum = vnd_docker_models_RestartPolicyNameEnum
  export type RootFsStorage = vnd_docker_models_RootFsStorage
  export type RootFsStorageSnapshot = vnd_docker_models_RootFsStorageSnapshot
  export type Storage = vnd_docker_models_Storage
  export type ThrottleDevice = vnd_docker_models_ThrottleDevice
}
