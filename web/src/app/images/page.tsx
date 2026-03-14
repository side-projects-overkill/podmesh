"use client";

import { useQuery } from "@tanstack/react-query";
import { api } from "@/lib/api";
import { formatBytes } from "@/lib/utils";

export default function ImagesPage() {
  const { data, isLoading } = useQuery({
    queryKey: ["images"],
    queryFn: api.images.list,
  });

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold">Images</h1>
          <p className="text-sm text-[var(--muted-foreground)]">Manage container images</p>
        </div>
        <button className="rounded-md bg-[var(--accent)] px-4 py-2 text-sm font-medium text-[var(--accent-foreground)] hover:opacity-90 transition-opacity">
          Pull Image
        </button>
      </div>

      <div className="rounded-lg border border-[var(--border)] bg-[var(--card)]">
        <div className="grid grid-cols-5 gap-4 border-b border-[var(--border)] p-3 text-xs font-medium uppercase tracking-wider text-[var(--muted-foreground)]">
          <div className="col-span-2">Name</div>
          <div>Size</div>
          <div>Node</div>
          <div>Actions</div>
        </div>

        {isLoading ? (
          <div className="p-8 text-center text-sm text-[var(--muted-foreground)]">Loading...</div>
        ) : data && Array.isArray(data) && data.length > 0 ? (
          <div className="divide-y divide-[var(--border)]">
            {data.map((img: Record<string, unknown>, i: number) => (
              <div key={i} className="grid grid-cols-5 gap-4 p-3 text-sm hover:bg-[var(--muted)]/50 transition-colors">
                <div className="col-span-2 font-medium truncate">{String(img.names ?? img.id ?? "—")}</div>
                <div className="text-[var(--muted-foreground)]">{formatBytes(Number(img.size) || 0)}</div>
                <div className="text-[var(--muted-foreground)]">{String(img.node_id ?? "—")}</div>
                <div className="flex gap-2">
                  <button className="text-xs text-[var(--accent)] hover:underline">Inspect</button>
                  <button className="text-xs text-[var(--destructive)] hover:underline">Remove</button>
                </div>
              </div>
            ))}
          </div>
        ) : (
          <div className="p-8 text-center text-sm text-[var(--muted-foreground)]">No images found.</div>
        )}
      </div>
    </div>
  );
}
