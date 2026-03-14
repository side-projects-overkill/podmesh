# PodMesh Architecture

## System Overview

```
┌─────────────────────────────────────────────────┐
│                    Browser                       │
│              (Next.js Frontend)                  │
└──────────────┬──────────────────┬────────────────┘
               │ REST             │ WebSocket
               ▼                  ▼
┌─────────────────────────────────────────────────┐
│              PodMesh Server                      │
│  ┌─────────┐ ┌──────────┐ ┌──────────────────┐  │
│  │ REST API│ │ WS Events│ │ OpenAPI/Swagger   │  │
│  └────┬────┘ └────┬─────┘ └──────────────────┘  │
│       │           │                              │
│  ┌────▼───────────▼─────────────────────────┐   │
│  │          Node Registry                    │   │
│  │    (tracks connected agents)              │   │
│  └────┬──────────┬──────────┬───────────┘   │   │
└───────┼──────────┼──────────┼───────────────┘   │
        │          │          │                    │
   ┌────▼───┐ ┌───▼────┐ ┌──▼─────┐              │
   │ Agent  │ │ Agent  │ │ Agent  │              │
   │ Node 1 │ │ Node 2 │ │ Node N │              │
   └───┬────┘ └───┬────┘ └───┬────┘              │
       │          │          │                    │
   ┌───▼────┐ ┌──▼─────┐ ┌──▼─────┐              │
   │Podman  │ │Podman  │ │Podman  │              │
   │Socket  │ │Socket  │ │Socket  │              │
   └────────┘ └────────┘ └────────┘
```

## Crate Dependency Graph

```
podmesh-core          (models, types, errors)
    ▲
    │
podmesh-client        (Podman socket API client)
    ▲
    ├──────────────┐
    │              │
podmesh-server   podmesh-agent
    │
podmesh-cli (via HTTP, not direct dependency)
```

## Data Flow

1. **User action** → Frontend sends REST request to Server
2. **Server** → Looks up target node → Forwards request to Agent
3. **Agent** → Calls Podman via Unix socket → Returns response
4. **Server** → Aggregates results → Returns to Frontend
5. **Events** → Agent streams Podman events → Server broadcasts via WebSocket → Frontend updates

## Design Decisions

### Why Rust?

- **Agent**: minimal memory footprint (~2-5MB RSS), single static binary,
  no runtime/GC, safety guarantees for concurrent socket I/O
- **Server**: high throughput, type-safe API layer, async-native with Tokio
- **CLI**: instant startup, single binary distribution

### Why axum?

- Built by the Tokio team, first-class async support
- Tower middleware ecosystem (tracing, CORS, compression)
- WebSocket support built-in
- Strong typing with extractors

### Why separate agent?

- Each node has its own Podman instance
- Agent runs with access to local Unix socket
- Server doesn't need direct socket access
- Enables multi-node orchestration
- Agents can be independently deployed and updated

## Future Architecture

### Plugin System (WASM)

```
Server
  └── Plugin Runtime (wasmtime)
       ├── Plugin A (custom metrics)
       ├── Plugin B (alerting)
       └── Plugin C (cost analysis)
```

### AI Operations

```
User: "show containers using most memory"
  → NL Parser → Query Builder → API Call → Response Formatter
```
