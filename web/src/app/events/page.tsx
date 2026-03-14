"use client";

import { useEventStream } from "@/hooks/use-websocket";

export default function EventsPage() {
  const { connected, events } = useEventStream();

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-2xl font-bold">Events</h1>
        <p className="text-sm text-[var(--muted-foreground)]">
          Live event stream
          <span className={`ml-2 ${connected ? "text-[var(--success)]" : "text-[var(--destructive)]"}`}>
            {connected ? "● Connected" : "● Disconnected"}
          </span>
        </p>
      </div>

      <div className="rounded-lg border border-[var(--border)] bg-[var(--card)]">
        {events.length > 0 ? (
          <div className="divide-y divide-[var(--border)]">
            {events.map((event) => (
              <div key={event.id} className="flex items-center gap-4 p-3 text-sm">
                <span className="rounded bg-[var(--muted)] px-2 py-0.5 text-xs font-mono text-[var(--accent)]">
                  {event.kind}
                </span>
                <span className="text-[var(--foreground)]">{event.actor_name ?? event.actor_id}</span>
                <span className="ml-auto text-xs text-[var(--muted-foreground)]">
                  {new Date(event.timestamp).toLocaleTimeString()}
                </span>
              </div>
            ))}
          </div>
        ) : (
          <div className="p-8 text-center text-sm text-[var(--muted-foreground)]">
            Waiting for events... Events will appear here in real-time when containers, pods, or images change.
          </div>
        )}
      </div>
    </div>
  );
}
