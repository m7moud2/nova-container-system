use anyhow::Result;
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

pub type Db = Arc<Mutex<Connection>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub name: String,
    pub billing_tier: String,
    pub api_key: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub status: String,
    pub commit_hash: String,
    pub updated_at: String,
    pub language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deployment {
    pub id: String,
    pub project_id: String,
    pub status: String,
    pub error_info: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Log {
    pub id: i64,
    pub project_id: String,
    pub deployment_id: String,
    pub level: String,
    pub message: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStats {
    pub active_containers: u32,
    pub health_status: String,
    pub memory_usage: String,
    pub cpu_usage_percent: u32,
    pub storage_usage_gb: f32,
    pub network_in_mb: f32,
    pub network_out_mb: f32,
}

/// Initialize SQLite database and seed demo data
pub fn init_db() -> Result<Db> {
    let conn = Connection::open("nova_dashboard.db")?;
    
    // Read and execute schema
    let schema = include_str!("schema.sql");
    conn.execute_batch(schema)?;
    
    Ok(Arc::new(Mutex::new(conn)))
}

/// Get user by email
pub fn get_user_by_email(db: &Db, email: &str) -> Result<Option<(User, String)>> {
    let conn = db.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, email, name, password_hash, billing_tier, api_key, created_at FROM users WHERE email = ?1"
    )?;
    
    let user = stmt.query_row(params![email], |row| {
        Ok((
            User {
                id: row.get(0)?,
                email: row.get(1)?,
                name: row.get(2)?,
                billing_tier: row.get(4)?,
                api_key: row.get(5)?,
                created_at: row.get(6)?,
            },
            row.get::<_, String>(3)?, // password_hash
        ))
    }).optional()?;
    
    Ok(user)
}

/// Get user by ID
pub fn get_user_by_id(db: &Db, user_id: i64) -> Result<Option<User>> {
    let conn = db.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, email, name, billing_tier, api_key, created_at FROM users WHERE id = ?1"
    )?;
    
    let user = stmt.query_row(params![user_id], |row| {
        Ok(User {
            id: row.get(0)?,
            email: row.get(1)?,
            name: row.get(2)?,
            billing_tier: row.get(3)?,
            api_key: row.get(4)?,
            created_at: row.get(5)?,
        })
    }).optional()?;
    
    Ok(user)
}

/// Create a new user
pub fn create_user(db: &Db, email: &str, password_hash: &str, name: &str) -> Result<i64> {
    let conn = db.lock().unwrap();
    let api_key = format!("nk_{}", uuid::Uuid::new_v4().to_string().replace("-", ""));
    conn.execute(
        "INSERT INTO users (email, password_hash, name, billing_tier, api_key) VALUES (?1, ?2, ?3, 'free', ?4)",
        params![email, password_hash, name, api_key],
    )?;
    
    Ok(conn.last_insert_rowid())
}

/// Update user profile
pub fn update_user(db: &Db, user_id: i64, name: &str) -> Result<()> {
    let conn = db.lock().unwrap();
    conn.execute(
        "UPDATE users SET name = ?1 WHERE id = ?2",
        params![name, user_id],
    )?;
    Ok(())
}

/// Delete user account
pub fn delete_user(db: &Db, user_id: i64) -> Result<()> {
    let conn = db.lock().unwrap();
    conn.execute("DELETE FROM users WHERE id = ?1", params![user_id])?;
    Ok(())
}

/// Get projects for a user
pub fn get_user_projects(db: &Db, user_id: i64) -> Result<Vec<Project>> {
    let conn = db.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, name, status, commit_hash, updated_at, language FROM projects WHERE user_id = ?1"
    )?;
    
    let projects = stmt.query_map(params![user_id], |row| {
        Ok(Project {
            id: row.get(0)?,
            name: row.get(1)?,
            status: row.get(2)?,
            commit_hash: row.get(3)?,
            updated_at: row.get(4)?,
            language: row.get(5)?,
        })
    })?
    .collect::<Result<Vec<_>, _>>()?;
    
    Ok(projects)
}

/// Create a new project for a user
pub fn create_project(
    db: &Db,
    user_id: i64,
    name: &str,
    status: &str,
    commit_hash: &str,
    updated_at: &str,
    language: &str,
) -> Result<String> {
    let conn = db.lock().unwrap();
    let id = format!("p_{}", uuid::Uuid::new_v4().to_string()[..8].to_string());
    
    let now = chrono::Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO projects (id, user_id, name, status, commit_hash, updated_at, language) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![id, user_id, name, status, commit_hash, now, language],
    )?;
    
    Ok(id)
}

/// Update project status
pub fn update_project_status(db: &Db, project_id: &str, new_status: &str) -> Result<()> {
    let conn = db.lock().unwrap();
    let now = chrono::Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE projects SET status = ?1, updated_at = ?3 WHERE id = ?2",
        params![new_status, project_id, now],
    )?;
    Ok(())
}

/// Create a new deployment record
pub fn create_deployment(db: &Db, project_id: &str, status: &str) -> Result<String> {
    let conn = db.lock().unwrap();
    let id = format!("d_{}", uuid::Uuid::new_v4().to_string()[..8].to_string());
    conn.execute(
        "INSERT INTO deployments (id, project_id, status) VALUES (?1, ?2, ?3)",
        params![id, project_id, status],
    )?;
    Ok(id)
}

/// Update deployment status and error info
pub fn update_deployment_status(db: &Db, deployment_id: &str, status: &str, error: Option<&str>) -> Result<()> {
    let conn = db.lock().unwrap();
    conn.execute(
        "UPDATE deployments SET status = ?1, error_info = ?2 WHERE id = ?3",
        params![status, error, deployment_id],
    )?;
    Ok(())
}

/// Get deployments for a project
pub fn get_project_deployments(db: &Db, project_id: &str) -> Result<Vec<Deployment>> {
    let conn = db.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, project_id, status, error_info, created_at FROM deployments WHERE project_id = ?1 ORDER BY created_at DESC"
    )?;
    
    let deployments = stmt.query_map(params![project_id], |row| {
        Ok(Deployment {
            id: row.get(0)?,
            project_id: row.get(1)?,
            status: row.get(2)?,
            error_info: row.get(3)?,
            created_at: row.get(4)?,
        })
    })?
    .collect::<Result<Vec<_>, _>>()?;
    
    Ok(deployments)
}

/// Add a new log entry
pub fn add_log(db: &Db, project_id: &str, deployment_id: &str, level: &str, message: &str) -> Result<()> {
    let conn = db.lock().unwrap();
    conn.execute(
        "INSERT INTO logs (project_id, deployment_id, level, message) VALUES (?1, ?2, ?3, ?4)",
        params![project_id, deployment_id, level, message],
    )?;
    Ok(())
}

/// Get logs for a project
pub fn get_project_logs(db: &Db, project_id: &str) -> Result<Vec<Log>> {
    let conn = db.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, project_id, deployment_id, level, message, timestamp FROM logs WHERE project_id = ?1 ORDER BY timestamp ASC"
    )?;
    
    let logs = stmt.query_map(params![project_id], |row| {
        Ok(Log {
            id: row.get(0)?,
            project_id: row.get(1)?,
            deployment_id: row.get(2)?,
            level: row.get(3)?,
            message: row.get(4)?,
            timestamp: row.get(5)?,
        })
    })?
    .collect::<Result<Vec<_>, _>>()?;
    
    Ok(logs)
}

/// Get project ID by name
pub fn get_project_id_by_name(db: &Db, name: &str) -> Result<Option<String>> {
    let conn = db.lock().unwrap();
    let mut stmt = conn.prepare("SELECT id FROM projects WHERE name = ?1")?;
    let id = stmt.query_row(params![name], |row| row.get(0)).optional()?;
    Ok(id)
}

/// Get system stats (real metrics)
pub fn get_stats(_db: &Db) -> SystemStats {
    use sysinfo::{System, Disks};
    
    let mut sys = System::new_all();
    sys.refresh_all();
    
    // Calculate CPU usage (average across all cores)
    let cpu_usage = sys.global_cpu_info().cpu_usage() as u32;
    
    // Memory usage
    let total_mem = sys.total_memory() as f32 / 1024.0 / 1024.0 / 1024.0; // GB
    let used_mem = sys.used_memory() as f32 / 1024.0 / 1024.0 / 1024.0; // GB
    let memory_usage_str = format!("{:.1} / {:.1} GB", used_mem, total_mem);
    
    // Storage usage (checking root or first disk)
    let disks = Disks::new_with_refreshed_list();
    let mut storage_gb = 0.0;
    if let Some(disk) = disks.first() {
        let used = (disk.total_space() - disk.available_space()) as f32 / 1024.0 / 1024.0 / 1024.0;
        storage_gb = used;
    }

    // Count active Nova processes
    let active_containers = sys.processes()
        .values()
        .filter(|p| p.name().to_string().to_lowercase().contains("nova"))
        .count() as u32;

    SystemStats {
        active_containers,
        health_status: "Healthy".to_string(),
        memory_usage: memory_usage_str,
        cpu_usage_percent: cpu_usage,
        storage_usage_gb: storage_gb,
        network_in_mb: 42.5 + (rand::random::<f32>() * 10.0), // Mock dynamic traffic
        network_out_mb: 28.1 + (rand::random::<f32>() * 5.0),
    }
}

/// Count total deployed projects
pub fn count_projects(db: &Db) -> Result<u64> {
    let conn = db.lock().unwrap();
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM projects")?;
    let count: i64 = stmt.query_row([], |row| row.get(0))?;
    Ok(count as u64)
}

/// Count total registered users
pub fn count_users(db: &Db) -> Result<u64> {
    let conn = db.lock().unwrap();
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM users")?;
    let count: i64 = stmt.query_row([], |row| row.get(0))?;
    Ok(count as u64)
}

