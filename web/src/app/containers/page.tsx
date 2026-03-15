"use client";

import { useState } from "react";
import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { api } from "@/lib/api";
import { Dialog } from "@/components/ui/dialog";

function CreateContainerDialog({
  open,
  onClose,
}: {
  open: boolean;
  onClose: () => void;
}) {
  const queryClient = useQueryClient();
  const [form, setForm] = useState({
    name: "",
    image: "",
    cmd: "",
    env: "",
  });
  const [error, setError] = useState<string | null>(null);

  const mutation = useMutation({
    mutationFn: () =>
      api.containers.create({
        name: form.name || undefined,
        image: form.image,
        cmd: form.cmd ? form.cmd.split(" ").filter(Boolean) : undefined,
        env: form.env ? form.env.split("\n").filter(Boolean) : undefined,
      }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["containers"] });
      setForm({ name: "", image: "", cmd: "", env: "" });
      setError(null);
      onClose();
    },
    onError: (err: Error) => setError(err.message),
  });

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!form.image.trim()) {
      setError("Image is required");
      return;
    }
    setError(null);
    mutation.mutate();
  };

  return (
    <Dialog open={open} onClose={onClose} title="Create Container">
      <form onSubmit={handleSubmit} className="space-y-4">
        <div>
          <label className="mb-1.5 block text-sm font-medium text-[var(--muted-foreground)]">
            Image <span className="text-[var(--destructive)]">*</span>
          </label>
          <input
            type="text"
            value={form.image}
            onChange={(e) => setForm((f) => ({ ...f, image: e.target.value }))}
            placeholder="docker.io/library/nginx:latest"
            className="w-full rounded-md border border-[var(--border)] bg-[var(--background)] px-3 py-2 text-sm text-[var(--foreground)] placeholder:text-[var(--muted-foreground)] focus:border-[var(--accent)] focus:outline-none focus:ring-1 focus:ring-[var(--accent)]"
          />
        </div>

        <div>
          <label className="mb-1.5 block text-sm font-medium text-[var(--muted-foreground)]">
            Name
          </label>
          <input
            type="text"
            value={form.name}
            onChange={(e) => setForm((f) => ({ ...f, name: e.target.value }))}
            placeholder="my-container"
            className="w-full rounded-md border border-[var(--border)] bg-[var(--background)] px-3 py-2 text-sm text-[var(--foreground)] placeholder:text-[var(--muted-foreground)] focus:border-[var(--accent)] focus:outline-none focus:ring-1 focus:ring-[var(--accent)]"
          />
        </div>

        <div>
          <label className="mb-1.5 block text-sm font-medium text-[var(--muted-foreground)]">
            Command
          </label>
          <input
            type="text"
            value={form.cmd}
            onChange={(e) => setForm((f) => ({ ...f, cmd: e.target.value }))}
            placeholder="/bin/sh -c 'echo hello'"
            className="w-full rounded-md border border-[var(--border)] bg-[var(--background)] px-3 py-2 text-sm text-[var(--foreground)] placeholder:text-[var(--muted-foreground)] focus:border-[var(--accent)] focus:outline-none focus:ring-1 focus:ring-[var(--accent)]"
          />
        </div>

        <div>
          <label className="mb-1.5 block text-sm font-medium text-[var(--muted-foreground)]">
            Environment Variables <span className="text-xs">(one per line, KEY=VALUE)</span>
          </label>
          <textarea
            value={form.env}
            onChange={(e) => setForm((f) => ({ ...f, env: e.target.value }))}
            placeholder={"FOO=bar\nDEBUG=true"}
            rows={3}
            className="w-full rounded-md border border-[var(--border)] bg-[var(--background)] px-3 py-2 text-sm text-[var(--foreground)] placeholder:text-[var(--muted-foreground)] focus:border-[var(--accent)] focus:outline-none focus:ring-1 focus:ring-[var(--accent)]"
          />
        </div>

        {error && (
          <p className="rounded-md bg-[var(--destructive)]/10 px-3 py-2 text-sm text-[var(--destructive)]">
            {error}
          </p>
        )}

        <div className="flex justify-end gap-3 pt-2">
          <button
            type="button"
            onClick={onClose}
            className="rounded-md border border-[var(--border)] px-4 py-2 text-sm font-medium text-[var(--muted-foreground)] hover:bg-[var(--muted)] transition-colors"
          >
            Cancel
          </button>
          <button
            type="submit"
            disabled={mutation.isPending}
            className="rounded-md bg-[var(--accent)] px-4 py-2 text-sm font-medium text-[var(--accent-foreground)] hover:opacity-90 transition-opacity disabled:opacity-50"
          >
            {mutation.isPending ? "Creating..." : "Create"}
          </button>
        </div>
      </form>
    </Dialog>
  );
}

function stateColor(state: string): string {
  switch (state) {
    case "running":
      return "bg-green-500/10 text-green-400";
    case "exited":
    case "stopped":
      return "bg-red-500/10 text-red-400";
    case "paused":
      return "bg-yellow-500/10 text-yellow-400";
    case "created":
      return "bg-blue-500/10 text-blue-400";
    default:
      return "bg-[var(--muted)] text-[var(--muted-foreground)]";
  }
}

export default function ContainersPage() {
  const [showCreate, setShowCreate] = useState(false);
  const queryClient = useQueryClient();

  const { data, isLoading } = useQuery({
    queryKey: ["containers", true],
    queryFn: () => api.containers.list(true),
  });

  const startMutation = useMutation({
    mutationFn: (id: string) => api.containers.start(id),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ["containers"] }),
  });

  const stopMutation = useMutation({
    mutationFn: (id: string) => api.containers.stop(id),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ["containers"] }),
  });

  const restartMutation = useMutation({
    mutationFn: (id: string) => api.containers.restart(id),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ["containers"] }),
  });

  const removeMutation = useMutation({
    mutationFn: (id: string) => api.containers.remove(id),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ["containers"] }),
  });

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold">Containers</h1>
          <p className="text-sm text-[var(--muted-foreground)]">Manage Podman containers across nodes</p>
        </div>
        <button
          onClick={() => setShowCreate(true)}
          className="rounded-md bg-[var(--accent)] px-4 py-2 text-sm font-medium text-[var(--accent-foreground)] hover:opacity-90 transition-opacity"
        >
          Create Container
        </button>
      </div>

      <div className="rounded-lg border border-[var(--border)] bg-[var(--card)]">
        <div className="grid grid-cols-6 gap-4 border-b border-[var(--border)] p-3 text-xs font-medium uppercase tracking-wider text-[var(--muted-foreground)]">
          <div>Name</div>
          <div>Image</div>
          <div>State</div>
          <div>Ports</div>
          <div>Node</div>
          <div>Actions</div>
        </div>

        {isLoading ? (
          <div className="p-8 text-center text-sm text-[var(--muted-foreground)]">Loading...</div>
        ) : data && Array.isArray(data) && data.length > 0 ? (
          <div className="divide-y divide-[var(--border)]">
            {data.map((c: Record<string, unknown>, i: number) => {
              const id = String(c.id ?? c.name ?? i);
              const state = String(c.state ?? "unknown");
              return (
                <div key={id} className="grid grid-cols-6 gap-4 p-3 text-sm hover:bg-[var(--muted)]/50 transition-colors">
                  <div className="font-medium truncate">{String(c.name ?? "—")}</div>
                  <div className="truncate text-[var(--muted-foreground)]">{String(c.image ?? "—")}</div>
                  <div>
                    <span className={`rounded-full px-2 py-0.5 text-xs font-medium ${stateColor(state)}`}>
                      {state}
                    </span>
                  </div>
                  <div className="text-[var(--muted-foreground)]">—</div>
                  <div className="text-[var(--muted-foreground)]">{String(c.node_id ?? "—")}</div>
                  <div className="flex gap-2">
                    {state !== "running" && (
                      <button
                        onClick={() => startMutation.mutate(id)}
                        className="text-xs text-[var(--success)] hover:underline"
                      >
                        Start
                      </button>
                    )}
                    {state === "running" && (
                      <>
                        <button
                          onClick={() => stopMutation.mutate(id)}
                          className="text-xs text-[var(--warning)] hover:underline"
                        >
                          Stop
                        </button>
                        <button
                          onClick={() => restartMutation.mutate(id)}
                          className="text-xs text-[var(--accent)] hover:underline"
                        >
                          Restart
                        </button>
                      </>
                    )}
                    <button
                      onClick={() => {
                        if (confirm(`Remove container ${String(c.name ?? id)}?`)) {
                          removeMutation.mutate(id);
                        }
                      }}
                      className="text-xs text-[var(--destructive)] hover:underline"
                    >
                      Remove
                    </button>
                  </div>
                </div>
              );
            })}
          </div>
        ) : (
          <div className="p-8 text-center text-sm text-[var(--muted-foreground)]">
            No containers found. Connect a node to see containers.
          </div>
        )}
      </div>

      <CreateContainerDialog open={showCreate} onClose={() => setShowCreate(false)} />
    </div>
  );
}
