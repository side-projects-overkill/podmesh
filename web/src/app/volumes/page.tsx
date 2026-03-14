"use client";

import { useQuery } from "@tanstack/react-query";
import { api } from "@/lib/api";

export default function VolumesPage() {
  const { data, isLoading } = useQuery({
    queryKey: ["volumes"],
    queryFn: api.volumes.list,
  });

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold">Volumes</h1>
          <p className="text-sm text-[var(--muted-foreground)]">Manage persistent storage</p>
        </div>
        <button className="rounded-md bg-[var(--accent)] px-4 py-2 text-sm font-medium text-[var(--accent-foreground)] hover:opacity-90 transition-opacity">
          Create Volume
        </button>
      </div>

      <div className="rounded-lg border border-[var(--border)] bg-[var(--card)]">
        <div className="grid grid-cols-5 gap-4 border-b border-[var(--border)] p-3 text-xs font-medium uppercase tracking-wider text-[var(--muted-foreground)]">
          <div>Name</div>
          <div>Driver</div>
          <div>Mountpoint</div>
          <div>Node</div>
          <div>Actions</div>
        </div>

        {isLoading ? (
          <div className="p-8 text-center text-sm text-[var(--muted-foreground)]">Loading...</div>
        ) : data && Array.isArray(data) && data.length > 0 ? (
          <div className="divide-y divide-[var(--border)]">
            {data.map((v: Record<string, unknown>, i: number) => (
              <div key={i} className="grid grid-cols-5 gap-4 p-3 text-sm hover:bg-[var(--muted)]/50 transition-colors">
                <div className="font-medium">{String(v.name ?? "—")}</div>
                <div className="text-[var(--muted-foreground)]">{String(v.driver ?? "local")}</div>
                <div className="truncate text-[var(--muted-foreground)]">{String(v.mountpoint ?? "—")}</div>
                <div className="text-[var(--muted-foreground)]">{String(v.node_id ?? "—")}</div>
                <div className="flex gap-2">
                  <button className="text-xs text-[var(--accent)] hover:underline">Inspect</button>
                  <button className="text-xs text-[var(--destructive)] hover:underline">Remove</button>
                </div>
              </div>
            ))}
          </div>
        ) : (
          <div className="p-8 text-center text-sm text-[var(--muted-foreground)]">No volumes found.</div>
        )}
      </div>
    </div>
  );
}
