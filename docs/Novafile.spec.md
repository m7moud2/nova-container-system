# Novafile Specification v1.0

## Overview
A `Novafile` is a text document that contains instructions for building a Nova container image. Similar to Dockerfile, but optimized for WebAssembly containers.

## Syntax

### FROM
Specifies the base image. Use `scratch` for empty base.

```dockerfile
FROM scratch
FROM nova/base:latest
```

### COPY
Copies files from host to image.

```dockerfile
COPY ./app.wasm /app.wasm
COPY ./data /data
```

### ENV
Sets environment variables.

```dockerfile
ENV PORT=8080
ENV DEBUG=true
```

### CMD
Default command to run when container starts.

```dockerfile
CMD ["nova", "run", "/app.wasm"]
```

### EXPOSE
Documents which ports the container listens on.

```dockerfile
EXPOSE 8080
EXPOSE 3000
```

### WORKDIR
Sets the working directory.

```dockerfile
WORKDIR /app
```

### LABEL
Adds metadata to the image.

```dockerfile
LABEL version="1.0"
LABEL maintainer="team@nova.sh"
```

## Example Novafile

```dockerfile
# Build a web server
FROM scratch

# Copy application
COPY ./server.wasm /app/server.wasm
COPY ./static /app/static

# Set environment
ENV PORT=8080
ENV LOG_LEVEL=info

# Expose port
EXPOSE 8080

# Set working directory
WORKDIR /app

# Run command
CMD ["nova", "run", "/app/server.wasm", "--port", "8080"]
```

## Build Command

```bash
# Build image
nova build -f Novafile -t myapp:latest

# Build with custom context
nova build -f Novafile -t myapp:v1.0 --context ./src

# Build without cache
nova build -f Novafile -t myapp:dev --no-cache
```

## Image Naming

Format: `[registry/]name[:tag]`

Examples:
- `myapp` (defaults to `myapp:latest`)
- `myapp:v1.0`
- `registry.nova.sh/myapp:latest`
- `localhost:5000/myapp:dev`

## Best Practices

1. **Use specific base images**: Avoid `latest` in production
2. **Minimize layers**: Combine commands when possible
3. **Order matters**: Put frequently changing instructions last
4. **Use .novaignore**: Exclude unnecessary files
5. **Label your images**: Add version and metadata

## .novaignore

Similar to `.dockerignore`, excludes files from build context:

```
# .novaignore
*.log
.git
node_modules
target/
```
