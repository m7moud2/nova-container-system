-- Users table
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    name TEXT NOT NULL,
    billing_tier TEXT DEFAULT 'free',
    api_key TEXT UNIQUE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Sessions table for JWT tracking
CREATE TABLE IF NOT EXISTS sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    token TEXT UNIQUE NOT NULL,
    expires_at DATETIME NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Projects table
CREATE TABLE IF NOT EXISTS projects (
    id TEXT PRIMARY KEY,
    user_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    status TEXT NOT NULL,
    commit_hash TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    language TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Deployments history table
CREATE TABLE IF NOT EXISTS deployments (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL,
    status TEXT NOT NULL, -- 'STARTING', 'SUCCESS', 'FAILED'
    error_info TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);

-- Logs table for container execution history
CREATE TABLE IF NOT EXISTS logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id TEXT NOT NULL,
    deployment_id TEXT NOT NULL,
    level TEXT NOT NULL, -- 'INFO', 'ERROR', 'SUCCESS'
    message TEXT NOT NULL,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);

-- Insert demo account (password: demo123)
-- Password hash generated with bcrypt for 'demo123'
INSERT OR IGNORE INTO users (id, email, password_hash, name, billing_tier, api_key, created_at)
VALUES (
    1,
    'demo@nova.cloud',
    '$2b$12$vwubS7169jJOBJQnNT8CjeCh.X5GX.qJHky5undN5vUFOmB.nWzfO',
    'Demo User',
    'pro',
    'nk_demo1234567890',
    datetime('now')
);

-- Insert demo projects
INSERT OR IGNORE INTO projects (id, user_id, name, status, commit_hash, updated_at, language)
VALUES 
    ('p_1', 1, 'api-gateway', 'production', '7a2b9d1', datetime('now'), 'node'),
    ('p_2', 1, 'worker-process', 'staging', 'c4f1e2a', datetime('now'), 'python'),
    ('p_3', 1, 'core-engine', 'dev', 'b9e8c7d', datetime('now'), 'rust');

-- Insert initial logs
INSERT OR IGNORE INTO logs (project_id, deployment_id, level, message)
VALUES 
    ('p_1', 'd_initial', 'SUCCESS', 'Project api-gateway deployed successfully to Nova Edge.'),
    ('p_1', 'd_initial', 'INFO', 'Instance started in 0.24ms.'),
    ('p_2', 'd_initial', 'INFO', 'Python environment initialized.'),
    ('p_3', 'd_initial', 'INFO', 'Wasm runtime optimized for high-performance execution.');
