export interface ApiResponse<T> {
  success: boolean;
  data: T | null;
  error: string | null;
}

export interface Node {
  id: string;
  name: string;
  endpoint: string;
  status: "online" | "offline" | "degraded" | "maintenance";
  podman_version: string | null;
  os: string | null;
  arch: string | null;
  labels: Record<string, string>;
  last_seen: string;
  created_at: string;
}

export interface Container {
  id: string;
  name: string;
  image: string;
  image_id: string;
  state: "created" | "running" | "paused" | "stopped" | "exited" | "removing" | "dead" | "unknown";
  status: string;
  created: string;
  started_at: string | null;
  ports: PortMapping[];
  labels: Record<string, string>;
  pod_id: string | null;
  node_id: string;
}

export interface PortMapping {
  host_ip: string | null;
  host_port: number;
  container_port: number;
  protocol: string;
}

export interface ContainerStats {
  container_id: string;
  cpu_percent: number;
  memory_usage_bytes: number;
  memory_limit_bytes: number;
  memory_percent: number;
  network_rx_bytes: number;
  network_tx_bytes: number;
  block_read_bytes: number;
  block_write_bytes: number;
  pids: number;
  timestamp: string;
}

export interface Pod {
  id: string;
  name: string;
  state: "created" | "running" | "stopped" | "exited" | "dead" | "degraded";
  created: string;
  infra_container_id: string | null;
  containers: PodContainer[];
  labels: Record<string, string>;
  node_id: string;
}

export interface PodContainer {
  id: string;
  name: string;
  status: string;
}

export interface Image {
  id: string;
  names: string[];
  digest: string;
  size: number;
  created: string;
  labels: Record<string, string>;
  node_id: string;
}

export interface Volume {
  name: string;
  driver: string;
  mountpoint: string;
  labels: Record<string, string>;
  options: Record<string, string>;
  created_at: string;
  node_id: string;
}

export interface Network {
  name: string;
  id: string;
  driver: string;
  network_interface: string | null;
  subnets: Subnet[];
  ipv6_enabled: boolean;
  internal: boolean;
  dns_enabled: boolean;
  labels: Record<string, string>;
  created: string;
  node_id: string;
}

export interface Subnet {
  subnet: string;
  gateway: string;
}

export interface PodMeshEvent {
  id: string;
  kind: string;
  actor_id: string;
  actor_name: string | null;
  node_id: string;
  attributes: Record<string, string>;
  timestamp: string;
}

export interface NodeStats {
  node_id: string;
  cpu_usage_percent: number;
  memory_used_bytes: number;
  memory_total_bytes: number;
  containers_running: number;
  containers_stopped: number;
  pods_running: number;
  timestamp: string;
}
