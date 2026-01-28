# Quick Start: Deploy Your First App on Nova

## Current Capabilities

Nova currently runs **WebAssembly (Wasm)** modules. Here's how to deploy different types of applications:

---

## 1. Rust Applications

### Simple Console App

```bash
# Create app
cat > hello.rs << 'EOF'
fn main() {
    println!("Hello from Nova! 🚀");
}
EOF

# Compile to Wasm
rustc --target wasm32-wasi hello.rs -o hello.wasm

# Run on Nova
nova run hello.wasm
```

### HTTP Server (Coming Soon)

```bash
# Compile server
rustc --target wasm32-wasi examples/http_server.rs -o server.wasm

# Run with networking
nova run server.wasm --expose 8080
```

---

## 2. C/C++ Applications

### Prerequisites
Install [wasi-sdk](https://github.com/WebAssembly/wasi-sdk/releases)

```bash
# Create C app
cat > app.c << 'EOF'
#include <stdio.h>
int main() {
    printf("C app running on Nova!\n");
    return 0;
}
EOF

# Compile
/path/to/wasi-sdk/bin/clang app.c -o app.wasm

# Run
nova run app.wasm
```

---

## 3. JavaScript/TypeScript (via AssemblyScript)

### Prerequisites
```bash
npm install -g assemblyscript
```

### Create App
```typescript
// app.ts
export function main(): void {
    console.log("TypeScript on Nova!");
}
```

### Compile & Run
```bash
# Compile
asc app.ts -o app.wasm

# Run
nova run app.wasm
```

---

## 4. Python (via Pyodide - Experimental)

Coming soon! Will support Python apps compiled to Wasm.

---

## Current Limitations

❌ **Not Yet Supported:**
- Direct HTML/CSS/JS websites (need HTTP server support)
- Node.js apps (without Wasm compilation)
- Native Docker images
- Database servers (PostgreSQL, MySQL)

✅ **Workarounds:**
- Compile to WebAssembly first
- Use static site generators → Wasm
- Wait for HTTP server support (coming in Phase 2)

---

## Roadmap

### Phase 2 (Next 3 Months)
- [ ] HTTP server support (WASI sockets)
- [ ] Static file serving
- [ ] Reverse proxy support
- [ ] Database connectors

### Phase 3 (6 Months)
- [ ] Full web framework support (Next.js, Django)
- [ ] Container-to-container networking
- [ ] Load balancing

---

## Examples Included

Check the `examples/` directory:
- `hello.wat` - Hello World
- `network_test.wat` - Inter-container communication
- `file_write.wat` - Filesystem operations
- `http_server.rs` - HTTP server (compile first)

---

## Need Help?

- [Documentation](docs/)
- [Discord](https://discord.gg/nova)
- [GitHub Issues](https://github.com/nova-container/core/issues)
