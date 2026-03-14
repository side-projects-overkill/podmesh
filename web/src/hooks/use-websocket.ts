"use client";

import { useEffect, useRef, useCallback, useState } from "react";
import type { PodMeshEvent } from "@/types";

const WS_URL = process.env.NEXT_PUBLIC_WS_URL || "ws://localhost:8090/ws/events";

export function useEventStream(onEvent?: (event: PodMeshEvent) => void) {
  const wsRef = useRef<WebSocket | null>(null);
  const [connected, setConnected] = useState(false);
  const [events, setEvents] = useState<PodMeshEvent[]>([]);

  const connect = useCallback(() => {
    const ws = new WebSocket(WS_URL);

    ws.onopen = () => setConnected(true);
    ws.onclose = () => {
      setConnected(false);
      setTimeout(connect, 3000);
    };

    ws.onmessage = (msg) => {
      try {
        const event: PodMeshEvent = JSON.parse(msg.data);
        setEvents((prev) => [event, ...prev].slice(0, 200));
        onEvent?.(event);
      } catch {
        // ignore malformed messages
      }
    };

    wsRef.current = ws;
  }, [onEvent]);

  useEffect(() => {
    connect();
    return () => wsRef.current?.close();
  }, [connect]);

  return { connected, events };
}
