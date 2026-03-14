"use client";

import { useQuery } from "@tanstack/react-query";
import { api } from "@/lib/api";

function StatCard({ label, value, color }: { label: string; value: string | number; color: string }) {
  return (
    <div className="rounded-lg border border-[var(--border)] bg-[var(--card)] p-4">
      <p className="text-sm text-[var(--muted-foreground)]">{label}</p>
      <p className={`mt-1 text-2xl font-bold ${color}`}>{value}</p>
    </div>
  );
}

export default function DashboardPage() {
  const health = useQuery({ queryKey: ["health"], queryFn: api.health });
  const containers = useQuery({ queryKey: ["containers"], queryFn: () => api.containers.list(true) });
  const pods = useQuery({ queryKey: ["pods"], queryFn: api.pods.list });
  const images = useQuery({ queryKey: ["images"], queryFn: api.images.list });
  const nodes = useQuery({ queryKey: ["nodes"], queryFn: api.nodes.list });

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-2xl font-bold">Dashboard</h1>
        <p className="text-sm text-[var(--muted-foreground)]">
          PodMesh control plane overview
          {health.data && (
            <span className="ml-2 text-[var(--success)]">
              — server {health.data.status} (v{health.data.version})
            </span>
          )}
        </p>
      </div>

      <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-4">
        <StatCard
          label="Containers"
          value={containers.data?.length ?? "—"}
          color="text-[var(--accent)]"
        />
        <StatCard
          label="Pods"
          value={pods.data?.length ?? "—"}
          color="text-purple-400"
        />
        <StatCard
          label="Images"
          value={images.data?.length ?? "—"}
          color="text-blue-400"
        />
        <StatCard
          label="Nodes"
          value={nodes.data?.length ?? "—"}
          color="text-green-400"
        />
      </div>

      <div className="grid grid-cols-1 gap-6 lg:grid-cols-2">
        <div className="rounded-lg border border-[var(--border)] bg-[var(--card)] p-4">
          <h2 className="mb-3 text-sm font-semibold text-[var(--muted-foreground)]">Recent Activity</h2>
          <p className="text-sm text-[var(--muted-foreground)]">
            Connect to a node to see live activity.
          </p>
        </div>

        <div className="rounded-lg border border-[var(--border)] bg-[var(--card)] p-4">
          <h2 className="mb-3 text-sm font-semibold text-[var(--muted-foreground)]">Node Health</h2>
          {nodes.data && nodes.data.length > 0 ? (
            <ul className="space-y-2">
              {(nodes.data as Array<{ id: string; name: string; status: string }>).map((node) => (
                <li key={node.id} className="flex items-center justify-between text-sm">
                  <span>{node.name}</span>
                  <span
                    className={
                      node.status === "online" ? "text-[var(--success)]" : "text-[var(--destructive)]"
                    }
                  >
                    {node.status}
                  </span>
                </li>
              ))}
            </ul>
          ) : (
            <p className="text-sm text-[var(--muted-foreground)]">No nodes registered.</p>
          )}
        </div>
      </div>
    </div>
  );
}
