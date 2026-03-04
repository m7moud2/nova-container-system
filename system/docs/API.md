# Nova API Documentation

## Command Line Interface

### `nova run`

Run a container from a file or directory.

**Usage:**
```bash
nova run <PATH> [OPTIONS]
```

**Arguments:**
- `<PATH>` - Path to file (.py, .js, .wasm) or directory

**Options:**
- `--replicas <N>` - Number of replicas to run (default: 1)
- `--memory <MB>` - Memory limit in MB
- `--fuel <N>` - CPU fuel limit (instructions)
- `--map-dir <HOST:GUEST>` - Map host directory to container

**Examples:**
```bash
# Run Python app
nova run app.py

# Run with memory limit
nova run app.py --memory 512

# Run with replicas
nova run app.wasm --replicas 10

# Run with directory mapping
nova run app.wasm --map-dir ./data:/data
```

---

### `nova build`

Build a container image from a Novafile.

**Usage:**
```bash
nova build [OPTIONS]
```

**Options:**
- `-f, --file <PATH>` - Path to Novafile (default: ./Novafile)
- `-t, --tag <TAG>` - Image tag (e.g., myapp:v1.0)
- `--context <PATH>` - Build context directory (default: .)

**Examples:**
```bash
# Build with default Novafile
nova build -t myapp:latest

# Build with custom Novafile
nova build -f custom.novafile -t myapp:v1.0

# Build with custom context
nova build -t myapp:dev --context ./src
```

---

### `nova serve`

Serve static files over HTTP.

**Usage:**
```bash
nova serve <PATH> [OPTIONS]
```

**Arguments:**
- `<PATH>` - Directory to serve

**Options:**
- `-p, --port <PORT>` - Port to serve on (default: 8080)

**Examples:**
```bash
# Serve on default port
nova serve ./dist

# Serve on custom port
nova serve ./public --port 3000
```

---

## Language Detection

Nova automatically detects the language and framework:

### Python
- **Detection**: `.py` extension
- **Frameworks**: Django (manage.py), Flask (from flask import)
- **Dependencies**: Automatically installs from requirements.txt

### Node.js
- **Detection**: `.js` extension or package.json
- **Frameworks**: Express, Next.js, React, Vue (from package.json)
- **Dependencies**: Automatically runs npm install

### WebAssembly
- **Detection**: `.wasm` or `.wat` extension
- **Execution**: Direct Wasmtime execution
- **Performance**: 240µs startup

---

## Resource Limits

### Memory Limit
```bash
nova run app.py --memory 512  # 512 MB limit
```

### CPU Fuel Limit
```bash
nova run app.wasm --fuel 1000000  # 1M instructions
```

---

## Exit Codes

- `0` - Success
- `1` - General error
- `2` - Command line parsing error
- `3` - Runtime not found (e.g., Python not installed)
- `4` - File not found
- `5` - Build error

---

## Environment Variables

- `NOVA_LOG_LEVEL` - Log level (debug, info, warn, error)
- `NOVA_CACHE_DIR` - Cache directory for builds

---

## Configuration File

Create `.novarc` in your home directory:

```toml
[defaults]
memory = 512
fuel = 1000000

[python]
auto_install_deps = true

[nodejs]
auto_install_deps = true
```

---

## Programmatic API (Future)

Coming in v1.0:

```rust
use nova::Runtime;

let runtime = Runtime::new();
runtime.run("app.py")?;
```
