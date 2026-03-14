# PodMesh

**Modern web-based control plane for Podman.**

PodMesh is a container infrastructure control plane that sits on top of Podman. It provides a clean web UI, REST API, CLI, and multi-node agent architecture for managing containers, pods, images, volumes, and networks — all Podman-native.

```
Browser → PodMesh Server → podmesh-agent → Podman API
                              (per node)     (socket)
```

## Architecture

| Component | Language | Purpose |
|---|---|---|
| `podmesh-server` | Rust (axum) | Central API server, WebSocket events, OpenAPI |
| `podmesh-agent` | Rust | Lightweight per-node daemon, talks to local Podman socket |
| `podmesh-cli` | Rust (clap) | Command-line interface for the PodMesh API |
| `podmesh-core` | Rust | Shared data models, types, error handling |
| `podmesh-client` | Rust | Podman REST API client over Unix socket |
| `web/` | Next.js + TypeScript | Dashboard UI with Tailwind CSS |

## Features

- **Container Management** — list, create, start, stop, restart, remove, inspect, logs, stats
- **Pod Management** — create, start, stop, remove, inspect pods
- **Image Management** — list, pull, remove, inspect images
- **Volume Management** — create, list, remove, inspect volumes
- **Network Management** — create, list, remove, inspect networks
- **Real-time Events** — WebSocket event stream from Podman
- **Multi-node** — connect multiple agents to a single server
- **OpenAPI** — full Swagger UI at `/swagger-ui`
- **CLI** — `podmesh` command-line tool for scripting

## Quick Start

### Prerequisites

- Rust 1.75+ (`rustup` recommended)
- Node.js 20+ and npm
- Podman 4+ (with socket enabled)

### Enable Podman Socket

```bash
# Enable rootless Podman socket
systemctl --user enable --now podman.socket

# Or rootful
sudo systemctl enable --now podman.socket
```

### Build

```bash
# Build all Rust crates
make build

# Install frontend dependencies
cd web && npm install
```

### Run

**Terminal 1 — Server:**
```bash
cargo run -p podmesh-server
# Starts on http://localhost:8090
# Swagger UI at http://localhost:8090/swagger-ui
```

**Terminal 2 — Agent:**
```bash
cargo run -p podmesh-agent -- --server-url http://localhost:8090
# Starts on http://localhost:8091
# Auto-registers with server
```

**Terminal 3 — Frontend:**
```bash
cd web && npm run dev
# Starts on http://localhost:3000
```

### CLI

```bash
# Install CLI
make install-cli

# Use
podmesh health
podmesh container list --all
podmesh node list
podmesh image pull docker.io/library/nginx:latest
podmesh pod create --name my-pod
```

## Project Structure

```
podmesh/
├── Cargo.toml                  # Workspace manifest
├── Makefile                    # Build automation
├── crates/
│   ├── podmesh-core/           # Shared models, types, errors
│   ├── podmesh-client/         # Podman Unix socket API client
│   ├── podmesh-server/         # Central API server (axum)
│   │   └── src/
│   │       ├── api/            # REST route handlers
│   │       ├── config/         # Server configuration
│   │       ├── state/          # Application state
│   │       └── ws/             # WebSocket handlers
│   ├── podmesh-agent/          # Per-node agent daemon
│   └── podmesh-cli/            # CLI tool
│       └── src/commands/       # CLI subcommands
├── web/                        # Next.js frontend
│   └── src/
│       ├── app/                # Pages (App Router)
│       ├── components/         # React components
│       ├── hooks/              # Custom hooks (WebSocket, etc.)
│       ├── lib/                # API client, utils, store
│       └── types/              # TypeScript type definitions
└── docs/                       # Documentation
```

## API

All endpoints are prefixed with `/api`. Full OpenAPI spec available at `/swagger-ui`.

| Method | Path | Description |
|---|---|---|
| GET | `/api/health` | Health check |
| GET | `/api/ready` | Readiness check |
| GET/POST | `/api/nodes` | List / register nodes |
| GET/POST | `/api/containers` | List / create containers |
| GET/DELETE | `/api/containers/{id}` | Get / remove container |
| POST | `/api/containers/{id}/start` | Start container |
| POST | `/api/containers/{id}/stop` | Stop container |
| POST | `/api/containers/{id}/restart` | Restart container |
| GET | `/api/containers/{id}/logs` | Container logs |
| GET | `/api/containers/{id}/stats` | Container stats |
| GET/POST | `/api/pods` | List / create pods |
| GET/DELETE | `/api/pods/{id}` | Get / remove pod |
| POST | `/api/pods/{id}/start` | Start pod |
| POST | `/api/pods/{id}/stop` | Stop pod |
| GET | `/api/images` | List images |
| GET/DELETE | `/api/images/{id}` | Get / remove image |
| POST | `/api/images/pull` | Pull image |
| GET/POST | `/api/volumes` | List / create volumes |
| GET/DELETE | `/api/volumes/{name}` | Get / remove volume |
| GET/POST | `/api/networks` | List / create networks |
| GET/DELETE | `/api/networks/{name}` | Get / remove network |
| GET | `/api/events` | List recent events |
| WS | `/ws/events` | Real-time event stream |

## Configuration

Configuration via environment variables:

| Variable | Default | Description |
|---|---|---|
| `PODMESH__HOST` | `0.0.0.0` | Server bind address |
| `PODMESH__PORT` | `8090` | Server port |
| `PODMESH__CORS_ORIGIN` | `http://localhost:3000` | Allowed CORS origin |
| `PODMESH_AGENT_ADDR` | `0.0.0.0:8091` | Agent bind address |
| `PODMESH_PODMAN_SOCKET` | `/run/podman/podman.sock` | Podman socket path |
| `PODMESH_SERVER_URL` | — | Server URL for agent registration |
| `PODMESH_NODE_NAME` | `default` | Agent node name |

## Development

```bash
# Check all crates compile
make check

# Run tests
make test

# Format code
make fmt

# Lint
make lint

# Generate docs
make docs

# Dev mode with auto-reload (requires cargo-watch)
make dev-server
make dev-agent
make dev-web
```

## Roadmap

- [ ] Persistent storage (SQLite/PostgreSQL)
- [ ] JWT authentication and RBAC
- [ ] Multi-user support
- [ ] Container exec (interactive terminal)
- [ ] Image build
- [ ] GitOps (deploy from YAML)
- [ ] WASM plugin system
- [ ] AI operations assistant (natural language → container operations)
- [ ] Metrics and monitoring dashboards
- [ ] Alerting and notifications

## License

Apache-2.0
