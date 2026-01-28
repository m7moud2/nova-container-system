# Nova Troubleshooting Guide

## Common Issues & Solutions

### Installation Issues

#### Problem: `nova: command not found`
**Solution:**
```bash
# Check if nova is in PATH
which nova

# If not found, add to PATH
export PATH="/usr/local/bin:$PATH"

# Or reinstall
curl -sSf https://raw.githubusercontent.com/m7moud2/nova-container-system/main/install.sh | sh
```

#### Problem: Permission denied when installing
**Solution:**
```bash
# Use sudo
sudo mv nova /usr/local/bin/

# Or install to user directory
mkdir -p ~/bin
mv nova ~/bin/
export PATH="$HOME/bin:$PATH"
```

---

### Python Runtime Issues

#### Problem: `Python not found`
**Solution:**
```bash
# macOS
brew install python3

# Ubuntu/Debian
sudo apt install python3 python3-pip

# CentOS
sudo yum install python3
```

#### Problem: `Failed to install Python dependencies`
**Solution:**
```bash
# Install manually
pip3 install -r requirements.txt

# Or use virtual environment
python3 -m venv venv
source venv/bin/activate
pip install -r requirements.txt
```

#### Problem: Flask/Django not detected
**Solution:**
```bash
# Make sure Flask is imported
# Add to your Python file:
from flask import Flask

# For Django, ensure manage.py exists
ls manage.py
```

---

### Node.js Runtime Issues

#### Problem: `Node.js not found`
**Solution:**
```bash
# macOS
brew install node

# Ubuntu/Debian
sudo apt install nodejs npm

# Or use nvm
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
nvm install node
```

#### Problem: `npm not found`
**Solution:**
```bash
# Usually comes with Node.js
sudo apt install npm

# Or reinstall Node.js
```

#### Problem: Dependencies not installing
**Solution:**
```bash
# Install manually
npm install

# Clear cache
npm cache clean --force
npm install
```

---

### WebAssembly Issues

#### Problem: `Invalid Wasm module`
**Solution:**
```bash
# Recompile with correct target
rustc --target wasm32-wasi app.rs -o app.wasm

# For Go
tinygo build -o app.wasm -target=wasi app.go

# Verify Wasm file
file app.wasm
```

#### Problem: `Memory limit exceeded`
**Solution:**
```bash
# Increase memory limit
nova run app.wasm --memory 1024

# Or optimize your code
```

---

### Build Issues

#### Problem: `Novafile not found`
**Solution:**
```bash
# Specify Novafile path
nova build -f path/to/Novafile -t myapp:latest

# Or create Novafile in current directory
cat > Novafile << 'EOF'
FROM scratch
COPY ./app.wasm /app.wasm
CMD ["nova", "run", "/app.wasm"]
EOF
```

#### Problem: `Build context too large`
**Solution:**
```bash
# Create .novaignore
cat > .novaignore << 'EOF'
node_modules/
target/
.git/
*.log
EOF
```

---

### Performance Issues

#### Problem: Slow startup
**Solution:**
```bash
# Check if using Wasm (should be 240µs)
nova run app.wasm

# Python/Node.js will be ~50ms (still fast!)

# Reduce dependencies
# Optimize code
```

#### Problem: High memory usage
**Solution:**
```bash
# Set memory limit
nova run app.py --memory 256

# Monitor usage
# Optimize your application
```

---

### Networking Issues

#### Problem: Port already in use
**Solution:**
```bash
# Find process using port
lsof -i :8080

# Kill process
kill -9 <PID>

# Or use different port
# Edit your app to use different port
```

#### Problem: Cannot connect to container
**Solution:**
```bash
# Make sure app binds to 0.0.0.0, not localhost
# Python Flask:
app.run(host='0.0.0.0', port=5000)

# Node.js:
app.listen(3000, '0.0.0.0')
```

---

### GitHub Issues

#### Problem: Git push requires password
**Solution:**
```bash
# Use Personal Access Token
# 1. Go to: https://github.com/settings/tokens
# 2. Generate new token
# 3. Use token as password

# Or use SSH
git remote set-url origin git@github.com:USERNAME/nova-container-system.git
```

#### Problem: GitHub Pages not working
**Solution:**
```bash
# 1. Check branch exists
git branch -a

# 2. Enable in Settings > Pages
# 3. Select gh-pages branch
# 4. Wait 2-3 minutes
```

---

## Getting Help

### Check Logs
```bash
# Enable debug logging
export NOVA_LOG_LEVEL=debug
nova run app.py
```

### Report Issues
- GitHub Issues: https://github.com/m7moud2/nova-container-system/issues
- Include:
  - Nova version
  - Operating system
  - Error message
  - Steps to reproduce

### Community
- Discord: (create your server)
- Discussions: https://github.com/m7moud2/nova-container-system/discussions

---

## FAQ

**Q: Is Nova production-ready?**
A: This is a beta release. Use with caution in production.

**Q: How is Nova different from Docker?**
A: Nova uses WebAssembly for 1000x faster startup and better portability.

**Q: Can I run Docker images in Nova?**
A: Not yet. Docker compatibility is planned for future releases.

**Q: What languages are supported?**
A: Python, Node.js, Rust, Go, C/C++ are working now. More coming soon.

**Q: Is Nova free?**
A: Yes! Nova is open source (MIT license) and free forever.

---

**Still having issues?** Open an issue on GitHub!
