# Nova System Architecture

## Overview
Nova is a high-performance container runtime and cloud platform built with Rust. It combines a CLI tool for developers with a backend server for the cloud dashboard.

## System Components

### 1. CLI (Command Line Interface)
- **EntryPoint**: `src/main.rs` (minimal wrapper).
- **Logic**: `src/cli.rs`.
- **Functionality**:
    - `nova run`: Execute containers (Wasm, Python, Node.js).
    - `nova build`: Build images from `Novafile`.
    - `nova deploy`: Deploy to cloud.
    - `nova dashboard`: Start the local cloud server.

### 2. Cloud Dashboard (Server)
- **Module**: `src/dashboard_api/`.
- **Tech Stack**: Rust (Axum) + SQLite (In-Memory).
- **Frontend**: `www/` (HTML/JS).
- **API Endpoints**:
    - `POST /api/login`
    - `GET /api/projects`
    - `GET /api/stats`

### 3. Core Runtime
- **Module**: `src/runtimes/`.
- **Engine**: Wasmtime for WebAssembly.
- **Support**: Native Python/Node.js execution via system commands.

## Directory Structure
```
nova_container_system/
├── scripts/            # Helper scripts (deploy, start)
├── src/
│   ├── cli.rs          # CLI Commands
│   ├── dashboard_api/  # Backend Server
│   ├── runtimes/       # Container Engines
│   └── main.rs         # Entry Point
├── www/                # Frontend Assets
└── Cargo.toml          # Dependencies
```

## Running the Project
- **CLI**: `cargo run -- [COMMAND]`
- **Dashboard**: `./scripts/start_server.sh`
