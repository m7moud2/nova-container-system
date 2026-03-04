# 🚀 Nova Container System

**A lightweight, production-ready container runtime system built in Rust with WebAssembly support.**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Website](https://img.shields.io/badge/website-live-green.svg)](https://m7moud2.github.io/nova-container-system)

---

## ✨ Features

### 🔐 **Authentication & User Management**
- **Real Authentication System** with SQLite database
- **Password Hashing** using bcrypt for security
- **JWT Tokens** for session management
- **Sign Up & Login** pages with validation
- **Demo Account**: `demo@nova.cloud` / `demo123`

### 🎯 **Core Capabilities**
- **WebAssembly Runtime** powered by Wasmtime
- **Multi-Language Support**: Python, Node.js, Rust, Go
- **Resource Isolation** with configurable limits
- **Hot Reloading** for rapid development
- **CLI Interface** for easy management

### 📊 **Web Dashboard**
- **Real-time Monitoring** of containers and resources
- **Project Management** with deployment tracking
- **User Profiles** with personalized data
- **System Statistics** (CPU, Memory, Storage)
- **Modern UI** with dark theme and animations

---

## 🚀 Quick Start

### Prerequisites
- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- SQLite (bundled automatically)
- **Linux Compatibility**: Supports all major distributions including Ubuntu, Debian, CentOS 7+, RHEL, Fedora, and Alpine (via static musl linking).

### Installation

```bash
# Clone the repository
git clone https://github.com/m7moud2/nova-container-system.git
cd nova-container-system

# Build the project
cargo build --release

# Run the dashboard server
./target/release/nova_container_system dashboard --port 3000
```

### Access the Dashboard

1. Open your browser to **http://localhost:3000/login.html**
2. Use demo credentials:
   - **Email**: `demo@nova.cloud`
   - **Password**: `demo123`
3. Or create a new account at **http://localhost:3000/signup.html**

---

## 📖 Usage

### CLI Commands

```bash
# Start the web dashboard
nova_container_system dashboard --port 3000

# Run a WebAssembly module
nova_container_system run <path-to-wasm-file>

# Deploy a project (coming soon)
nova_container_system deploy <project-name>
```

### API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/api/signup` | Create new user account |
| POST | `/api/login` | Authenticate user |
| GET | `/api/me` | Get current user profile |
| GET | `/api/projects` | List user's projects |
| GET | `/api/stats` | Get system statistics |
| POST | `/api/deployments` | Create new deployment |

---

## 🏗️ Architecture

```
nova_container_system/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── cli.rs               # Command-line interface
│   ├── runtime/             # WebAssembly runtime
│   ├── runtimes/            # Language-specific runtimes
│   ├── runtime_detector.rs  # Auto-detect project type
│   └── dashboard_api/       # Web API & authentication
│       ├── auth.rs          # JWT & password hashing
│       ├── db.rs            # SQLite database layer
│       ├── handlers.rs      # API request handlers
│       └── schema.sql       # Database schema
├── www/                     # Frontend assets
│   ├── login.html           # Login page
│   ├── signup.html          # Sign up page
│   ├── dashboard.html       # Main dashboard
│   ├── dashboard.css        # Dashboard styles
│   ├── style.css            # Auth pages styles
│   └── js/
│       └── dashboard.js     # Dashboard logic
└── scripts/                 # Helper scripts
    ├── start_server.sh      # Quick server start
    └── deploy.sh            # Deployment script
```

---

## 🔒 Security

- **Password Hashing**: bcrypt with cost factor 12
- **JWT Tokens**: Secure session management with expiration
- **SQL Injection Protection**: Parameterized queries
- **CORS**: Configured for production use

> ⚠️ **Production Note**: Change the JWT secret in `src/dashboard_api/auth.rs` before deploying to production!

---

## 🛠️ Development

### Running in Development Mode

```bash
# Build and run with hot reload
cargo watch -x 'run -- dashboard --port 3000'

# Run tests
cargo test

# Check for issues
cargo clippy
```

### Database Management

```bash
# View database contents
sqlite3 nova_dashboard.db

# Reset database
rm nova_dashboard.db
./target/debug/nova_container_system dashboard --port 3000
```

---

## 📚 Documentation

- **[Architecture Guide](ARCHITECTURE.md)** - System design and components
- **[API Documentation](docs/API.md)** - Complete API reference (coming soon)
- **[Deployment Guide](docs/DEPLOYMENT.md)** - Production deployment (coming soon)

---

## 🌐 Live Demo

Visit our live website: **[https://m7moud2.github.io/nova-container-system](https://m7moud2.github.io/nova-container-system)**

---

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

---

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 🙏 Acknowledgments

- **Wasmtime** - WebAssembly runtime
- **Axum** - Web framework
- **Rusqlite** - SQLite bindings
- **bcrypt** - Password hashing
- **jsonwebtoken** - JWT implementation

---

## 📧 Contact

**Mahmoud Abdelkawy** - [@m7moud2](https://github.com/m7moud2)

Project Link: [https://github.com/m7moud2/nova-container-system](https://github.com/m7moud2/nova-container-system)

---

<div align="center">
  <strong>Built with ❤️ using Rust</strong>
</div>
