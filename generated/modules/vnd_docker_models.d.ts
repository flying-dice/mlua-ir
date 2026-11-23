// ===== vnd:docker_models =====

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

interface vnd_docker_models_PortSummary {
  IP?: string
  PrivatePort: number
  PublicPort?: number
  Type?: string
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
  /** One line for a container: its first name and its state. Takes bollard's type as an argument too. */
  export function line(this: void, container: vnd_docker_models_ContainerSummary): string
  export type ContainerSummary = vnd_docker_models_ContainerSummary
  export type ContainerSummaryHealth = vnd_docker_models_ContainerSummaryHealth
  export type ContainerSummaryHealthStatusEnum = vnd_docker_models_ContainerSummaryHealthStatusEnum
  export type ContainerSummaryHostConfig = vnd_docker_models_ContainerSummaryHostConfig
  export type ContainerSummaryNetworkSettings = vnd_docker_models_ContainerSummaryNetworkSettings
  export type ContainerSummaryStateEnum = vnd_docker_models_ContainerSummaryStateEnum
  export type EndpointIpamConfig = vnd_docker_models_EndpointIpamConfig
  export type EndpointSettings = vnd_docker_models_EndpointSettings
  export type MountPoint = vnd_docker_models_MountPoint
  export type MountPointTypeEnum = vnd_docker_models_MountPointTypeEnum
  export type OciDescriptor = vnd_docker_models_OciDescriptor
  export type OciPlatform = vnd_docker_models_OciPlatform
  export type PortSummary = vnd_docker_models_PortSummary
}
