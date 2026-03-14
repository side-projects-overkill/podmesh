import type { ApiResponse } from "@/types";

const API_BASE = process.env.NEXT_PUBLIC_API_URL || "";

async function request<T>(path: string, options?: RequestInit): Promise<T> {
  const res = await fetch(`${API_BASE}${path}`, {
    ...options,
    headers: {
      "Content-Type": "application/json",
      ...options?.headers,
    },
  });

  if (!res.ok) {
    throw new Error(`API error: ${res.status} ${res.statusText}`);
  }

  const json: ApiResponse<T> = await res.json();

  if (!json.success) {
    throw new Error(json.error || "Unknown API error");
  }

  return json.data as T;
}

export const api = {
  nodes: {
    list: () => request<unknown[]>("/api/nodes"),
    register: (name: string, endpoint: string) =>
      request<unknown>("/api/nodes", {
        method: "POST",
        body: JSON.stringify({ name, endpoint }),
      }),
  },

  containers: {
    list: (all = false) => request<unknown[]>(`/api/containers?all=${all}`),
    get: (id: string) => request<unknown>(`/api/containers/${id}`),
    start: (id: string) => request<unknown>(`/api/containers/${id}/start`, { method: "POST" }),
    stop: (id: string) => request<unknown>(`/api/containers/${id}/stop`, { method: "POST" }),
    restart: (id: string) => request<unknown>(`/api/containers/${id}/restart`, { method: "POST" }),
    remove: (id: string) => request<unknown>(`/api/containers/${id}`, { method: "DELETE" }),
    logs: (id: string) => request<unknown>(`/api/containers/${id}/logs`),
    stats: (id: string) => request<unknown>(`/api/containers/${id}/stats`),
  },

  pods: {
    list: () => request<unknown[]>("/api/pods"),
    get: (id: string) => request<unknown>(`/api/pods/${id}`),
    create: (name: string) =>
      request<unknown>("/api/pods", { method: "POST", body: JSON.stringify({ name }) }),
    start: (id: string) => request<unknown>(`/api/pods/${id}/start`, { method: "POST" }),
    stop: (id: string) => request<unknown>(`/api/pods/${id}/stop`, { method: "POST" }),
    remove: (id: string) => request<unknown>(`/api/pods/${id}`, { method: "DELETE" }),
  },

  images: {
    list: () => request<unknown[]>("/api/images"),
    get: (id: string) => request<unknown>(`/api/images/${id}`),
    pull: (reference: string) =>
      request<unknown>("/api/images/pull", {
        method: "POST",
        body: JSON.stringify({ reference }),
      }),
    remove: (id: string) => request<unknown>(`/api/images/${id}`, { method: "DELETE" }),
  },

  volumes: {
    list: () => request<unknown[]>("/api/volumes"),
    get: (name: string) => request<unknown>(`/api/volumes/${name}`),
    create: (name: string) =>
      request<unknown>("/api/volumes", { method: "POST", body: JSON.stringify({ name }) }),
    remove: (name: string) => request<unknown>(`/api/volumes/${name}`, { method: "DELETE" }),
  },

  networks: {
    list: () => request<unknown[]>("/api/networks"),
    get: (name: string) => request<unknown>(`/api/networks/${name}`),
    create: (name: string) =>
      request<unknown>("/api/networks", { method: "POST", body: JSON.stringify({ name }) }),
    remove: (name: string) => request<unknown>(`/api/networks/${name}`, { method: "DELETE" }),
  },

  events: {
    list: () => request<unknown[]>("/api/events"),
  },

  health: () => request<{ status: string; version: string }>("/api/health"),
};
