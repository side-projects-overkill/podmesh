"use client";

import { useQuery } from "@tanstack/react-query";
import { api } from "@/lib/api";

export default function NodesPage() {
  const { data, isLoading } = useQuery({
    queryKey: ["nodes"],
    queryFn: api.nodes.list,
  });

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold">Nodes</h1>
          <p className="text-sm text-[var(--muted-foreground)]">Manage connected PodMesh agents</p>
        </div>
        <button className="rounded-md bg-[var(--accent)] px-4 py-2 text-sm font-medium text-[var(--accent-foreground)] hover:opacity-90 transition-opacity">
          Add Node
        </button>
      </div>

      <div className="rounded-lg border border-[var(--border)] bg-[var(--card)]">
        <div className="grid grid-cols-5 gap-4 border-b border-[var(--border)] p-3 text-xs font-medium uppercase tracking-wider text-[var(--muted-foreground)]">
          <div>Name</div>
          <div>Endpoint</div>
          <div>Status</div>
          <div>Last Seen</div>
          <div>Actions</div>
        </div>

        {isLoading ? (
          <div className="p-8 text-center text-sm text-[var(--muted-foreground)]">Loading...</div>
        ) : data && Array.isArray(data) && data.length > 0 ? (
          <div className="divide-y divide-[var(--border)]">
            {data.map((n: Record<string, unknown>, i: number) => (
              <div key={i} className="grid grid-cols-5 gap-4 p-3 text-sm hover:bg-[var(--muted)]/50 transition-colors">
                <div className="font-medium">{String(n.name ?? "—")}</div>
                <div className="truncate text-[var(--muted-foreground)]">{String(n.endpoint ?? "—")}</div>
                <div>
                  <span
                    className={`rounded-full px-2 py-0.5 text-xs font-medium ${
                      n.status === "online"
                        ? "bg-green-500/10 text-green-400"
                        : "bg-red-500/10 text-red-400"
                    }`}
                  >
                    {String(n.status ?? "unknown")}
                  </span>
                </div>
                <div className="text-[var(--muted-foreground)]">
                  {n.last_seen ? new Date(String(n.last_seen)).toLocaleString() : "—"}
                </div>
                <div>
                  <button className="text-xs text-[var(--destructive)] hover:underline">Remove</button>
                </div>
              </div>
            ))}
          </div>
        ) : (
          <div className="p-8 text-center text-sm text-[var(--muted-foreground)]">
            No nodes registered. Run <code className="text-[var(--accent)]">podmesh-agent</code> on your nodes.
          </div>
        )}
      </div>
    </div>
  );
}
