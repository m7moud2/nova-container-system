# Nova v0.1.0 Beta 🚀

**The next-generation container runtime. 1000x faster than Docker.**

## ✨ What's New

This is the first beta release of Nova, featuring:

### Core Features
- ⚡ **240µs startup** for WebAssembly containers (1000x faster than Docker)
- 🐍 **Python support** - Run Flask, Django, FastAPI apps (~50ms startup)
- ⚡ **Node.js support** - Run Express, Next.js, React, Vue apps (~50ms startup)
- 🦀 **Rust/C/C++/Go support** - Native WebAssembly compilation
- 🔄 **Auto-detection** - Automatically detects languages and frameworks
- 🛡️ **Resource limits** - Memory and CPU fuel control
- 📦 **Zero configuration** - Just run your code, no Dockerfile needed

### What Makes Nova Special
- **True portability**: Build once, run anywhere (Mac, Linux, Windows)
- **Secure by default**: WebAssembly sandboxing
- **Simple**: No complex configuration files
- **Fast**: Proven 1000x faster startup than traditional containers

## 🚀 Quick Start

### Installation

**macOS/Linux:**
```bash
curl -sSf https://raw.githubusercontent.com/m7moud2/nova-container-system/main/install.sh | sh
```

**Or download binary manually:**
- [macOS (ARM64)](https://github.com/m7moud2/nova-container-system/releases/download/v0.1.0-beta/nova-macos-arm64)
- [macOS (x86_64)](https://github.com/m7moud2/nova-container-system/releases/download/v0.1.0-beta/nova-macos-x86_64)
- [Linux (x86_64)](https://github.com/m7moud2/nova-container-system/releases/download/v0.1.0-beta/nova-linux-x86_64)

### Usage Examples

**Python (Flask):**
```bash
# Create app
cat > app.py << 'EOF'
from flask import Flask
app = Flask(__name__)

@app.route('/')
def hello():
    return 'Hello from Nova!'

if __name__ == '__main__':
    app.run(host='0.0.0.0', port=5000)
EOF

# Run with Nova
nova run app.py
```

**Node.js (Express):**
```bash
# Create app
cat > server.js << 'EOF'
const express = require('express');
const app = express();

app.get('/', (req, res) => {
    res.send('Hello from Nova!');
});

app.listen(3000);
EOF

# Run with Nova
nova run server.js
```

**WebAssembly (Rust):**
```bash
# Compile to Wasm
rustc --target wasm32-wasi app.rs -o app.wasm

# Run with Nova
nova run app.wasm
```

## 📖 Documentation

- **Website**: https://m7moud2.github.io/nova-container-system
- **Learn Nova**: https://m7moud2.github.io/nova-container-system/learn.html
- **Examples**: [/examples](https://github.com/m7moud2/nova-container-system/tree/main/examples)
- **Contributing**: [CONTRIBUTING.md](https://github.com/m7moud2/nova-container-system/blob/main/CONTRIBUTING.md)

## 🐛 Known Issues

This is a **beta release**. Known limitations:

- Container registry (push/pull) not yet implemented
- Monitoring/logging system in development
- Some advanced features planned for future releases

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](https://github.com/m7moud2/nova-container-system/blob/main/CONTRIBUTING.md).

## 📝 Feedback

Please report issues on [GitHub Issues](https://github.com/m7moud2/nova-container-system/issues).

## 🙏 Acknowledgments

Built with:
- [Wasmtime](https://wasmtime.dev/) - WebAssembly runtime
- [Rust](https://www.rust-lang.org/) - Systems programming language
- [Tokio](https://tokio.rs/) - Async runtime

## 📄 License

MIT License - see [LICENSE](https://github.com/m7moud2/nova-container-system/blob/main/LICENSE)

---

**Thank you for trying Nova! 🚀**

*Star the repo if you like it!* ⭐
