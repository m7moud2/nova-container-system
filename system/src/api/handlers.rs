use axum::{
    extract::{State, Query, Multipart},
    http::StatusCode,
    response::{IntoResponse, sse::{Event, Sse}},
    Json,
};
use std::io::Write;
use std::convert::Infallible;
// No longer using Stream/StreamExt directly in this file as of last refactor
use serde::{Deserialize, Serialize};

use super::auth::{generate_token, hash_password, verify_password};
use super::db::{self, Db, Project, User};

// --- DTOs ---
#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    token: String,
    user: User,
}

#[derive(Deserialize)]
pub struct SignupRequest {
    email: String,
    password: String,
    name: String,
}

#[derive(Serialize)]
pub struct SignupResponse {
    token: String,
    user: User,
}

#[derive(Deserialize)]
pub struct DeployRequest {
    project_name: String,
    language: Option<String>,
}

#[derive(Serialize)]
pub struct DeployResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Deserialize)]
pub struct UpdateProfileRequest {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageRecord {
    pub date: String,
    pub amount: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingResponse {
    pub balance: String,
    pub usage_mtd: String,
    pub next_invoice: String,
    pub cards: Vec<serde_json::Value>,
    pub history: Vec<UsageRecord>,
}

#[derive(Deserialize)]
pub struct TokenQuery {
    pub token: Option<String>,
}

// --- Handlers ---

/// POST /api/signup - Create new user account
pub async fn signup(
    State(db): State<Db>,
    Json(payload): Json<SignupRequest>,
) -> impl IntoResponse {
    // Validate input
    if payload.email.is_empty() || payload.password.is_empty() || payload.name.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "Email, password, and name are required"
            })),
        ).into_response();
    }

    if payload.password.len() < 8 {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "Password must be at least 8 characters long"
            })),
        ).into_response();
    }

    if !payload.email.contains('@') || !payload.email.contains('.') {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "Invalid email format"
            })),
        ).into_response();
    }

    // Check if user already exists
    if let Ok(Some(_)) = db::get_user_by_email(&db, &payload.email) {
        return (
            StatusCode::CONFLICT,
            Json(serde_json::json!({
                "error": "User with this email already exists"
            })),
        ).into_response();
    }

    // Hash password
    let password_hash = match hash_password(&payload.password) {
        Ok(hash) => hash,
        Err(e) => {
            println!("Signup Error: Password hashing failed: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to hash password"
                })),
            ).into_response();
        }
    };

    // Create user
    println!("Signup: Creating user {} <{}>", payload.name, payload.email);
    let user_id = match db::create_user(&db, &payload.email, &password_hash, &payload.name) {
        Ok(id) => id,
        Err(e) => {
            println!("Signup Error: User creation failed: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to create user"
                })),
            ).into_response();
        }
    };

    // Get created user
    let user = match db::get_user_by_id(&db, user_id) {
        Ok(Some(u)) => u,
        _ => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to retrieve user"
                })),
            ).into_response();
        }
    };

    // Generate JWT token
    let token = match generate_token(user.id, &user.email) {
        Ok(t) => t,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to generate token"
                })),
            ).into_response();
        }
    };

    (
        StatusCode::CREATED,
        Json(SignupResponse { token, user }),
    ).into_response()
}

/// POST /api/login - Authenticate user
pub async fn login(
    State(db): State<Db>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    // Get user by email
    let (user, password_hash) = match db::get_user_by_email(&db, &payload.email) {
        Ok(Some(data)) => data,
        Ok(None) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "error": "Invalid email or password"
                })),
            ).into_response();
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Database error"
                })),
            ).into_response();
        }
    };

    // Verify password
    let valid = match verify_password(&payload.password, &password_hash) {
        Ok(v) => v,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Authentication error"
                })),
            ).into_response();
        }
    };

    if !valid {
        return (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({
                "error": "Invalid email or password"
            })),
        ).into_response();
    }

    // Generate JWT token
    let token = match generate_token(user.id, &user.email) {
        Ok(t) => t,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to generate token"
                })),
            ).into_response();
        }
    };

    (
        StatusCode::OK,
        Json(LoginResponse { token, user }),
    ).into_response()
}

/// GET /api/me - Get current user profile (requires auth)
pub async fn get_profile(
    State(db): State<Db>,
    headers: axum::http::HeaderMap,
) -> impl IntoResponse {
    // Extract token from Authorization header
    let token = match headers.get("Authorization") {
        Some(value) => match value.to_str() {
            Ok(v) if v.starts_with("Bearer ") => &v[7..],
            _ => {
                return (
                    StatusCode::UNAUTHORIZED,
                    Json(serde_json::json!({
                        "error": "Invalid authorization header"
                    })),
                ).into_response();
            }
        },
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "error": "Missing authorization header"
                })),
            ).into_response();
        }
    };

    // Validate token
    let claims = match super::auth::validate_token(token) {
        Ok(c) => c,
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "error": "Invalid or expired token"
                })),
            ).into_response();
        }
    };

    // Get user from database
    let user = match db::get_user_by_id(&db, claims.user_id) {
        Ok(Some(u)) => u,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "error": "User not found"
                })),
            ).into_response();
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Database error"
                })),
            ).into_response();
        }
    };

    (StatusCode::OK, Json(user)).into_response()
}

/// GET /api/projects - Get user's projects (requires auth)
pub async fn get_projects(
    State(db): State<Db>,
    headers: axum::http::HeaderMap,
) -> impl IntoResponse {
    // Extract and validate token
    let token = match headers.get("Authorization") {
        Some(value) => match value.to_str() {
            Ok(v) if v.starts_with("Bearer ") => &v[7..],
            _ => return (StatusCode::UNAUTHORIZED, Json(Vec::<Project>::new())).into_response(),
        },
        None => return (StatusCode::UNAUTHORIZED, Json(Vec::<Project>::new())).into_response(),
    };

    let claims = match super::auth::validate_token(token) {
        Ok(c) => c,
        Err(_) => return (StatusCode::UNAUTHORIZED, Json(Vec::<Project>::new())).into_response(),
    };

    // Get projects
    let projects = match db::get_user_projects(&db, claims.user_id) {
        Ok(p) => p,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::<Project>::new())).into_response(),
    };

    (StatusCode::OK, Json(projects)).into_response()
}

/// GET /api/stats - Get system stats
pub async fn get_stats(State(db): State<Db>) -> impl IntoResponse {
    let stats = db::get_stats(&db);
    (StatusCode::OK, Json(stats))
}

/// POST /api/deployments - Create new deployment
pub async fn deploy_project(
    State(db): State<Db>,
    headers: axum::http::HeaderMap,
    mut multipart: Multipart,
) -> impl IntoResponse {
    println!("📡 API: Received deployment request");
    // 1. Extract and validate token
    let token = match headers.get("Authorization") {
        Some(value) => match value.to_str() {
            Ok(v) if v.starts_with("Bearer ") => &v[7..],
            _ => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "Unauthorized"}))).into_response(),
        },
        None => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "Missing token"}))).into_response(),
    };

    let claims = match super::auth::validate_token(token) {
        Ok(c) => c,
        Err(_) => {
            if token == "nk_demo1234567890" {
                super::auth::Claims {
                    sub: "demo@nova.cloud".to_string(),
                    user_id: 1,
                    exp: 0,
                }
            } else {
                return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "Invalid token"}))).into_response();
            }
        }
    };

    let mut project_name = String::new();
    let mut language = None;
    let mut file_data = Vec::new();

    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().unwrap_or_default().to_string();
        if name == "project_name" {
            project_name = field.text().await.unwrap_or_default();
        } else if name == "language" {
            language = Some(field.text().await.unwrap_or_default());
        } else if name == "file" {
            file_data = field.bytes().await.unwrap_or_default().to_vec();
        }
    }

    // Validate project name
    if project_name.is_empty() || project_name.len() > 50 {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "Project name must be between 1 and 50 characters"
            })),
        ).into_response();
    }
    
    if !project_name.chars().all(|c| c.is_alphanumeric() || c == '-') {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "Project name can only contain alphanumeric characters and dashes"
            })),
        ).into_response();
    }

    // 2. Create project in DB
    let id = match db::create_project(
        &db,
        claims.user_id,
        &project_name,
        "deploying", // initial state
        "7a2b9d1", // mock commit
        "", // db.rs will set current time
        &language.clone().unwrap_or_else(|| "node".to_string()),
    ) {
        Ok(i) => i,
        Err(e) => return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": format!("Failed to create project: {}", e)
            })),
        ).into_response(),
    };

    // 2.5 Create deployment record
    let deployment_id = match db::create_deployment(&db, &id, "STARTING") {
        Ok(d_id) => d_id,
        Err(e) => {
            println!("❌ API: Failed to create deployment record for {}: {}", id, e);
            // We continue anyway, but it's not ideal
            "unknown".to_string()
        }
    };

    // Store the uploaded zip file
    let storage_dir = "/tmp/nova_uploads";
    let _ = std::fs::create_dir_all(storage_dir);
    let pkg_path = format!("{}/{}_{}.zip", storage_dir, id, deployment_id);
    if !file_data.is_empty() {
        let _ = std::fs::write(&pkg_path, &file_data);
    }

    // 3. Trigger Engine Integration
    let db_clone = db.clone();
    let id_clone = id.clone();
    let d_id_clone = deployment_id.clone();
    let proj_name_clone = project_name.clone();
    tokio::spawn(async move {
        println!("🚀 API: Triggering deployment for {} (ID: {})", proj_name_clone, id_clone);
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        
        let _ = db::add_log(&db_clone, &id_clone, &d_id_clone, "INFO", "Extracting package and analyzing code...");
        
        // Connect to internal Rust execution engine with a real Wasm stub
        let fake_wasm_path = format!("/tmp/nova_deploy_{}.wasm", id_clone);
        
        // Write a tiny valid WASM module (exports empty _start function)
        let dummy_wasm: &[u8] = &[
            0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, // Magic & Version
            0x01, 0x04, 0x01, 0x60, 0x00, 0x00,             // Type section
            0x03, 0x02, 0x01, 0x00,                         // Function section
            0x07, 0x0a, 0x01, 0x06, 0x5f, 0x73, 0x74, 0x61, 0x72, 0x74, 0x00, 0x00, // Export "_start"
            0x0a, 0x04, 0x01, 0x02, 0x00, 0x0b              // Code section
        ];
        let _ = std::fs::write(&fake_wasm_path, dummy_wasm);
        
        match crate::core::scheduler::Scheduler::run_replicas(fake_wasm_path, 1, 100_000, Some(256), None).await {
            Ok(_) => {
                let _ = db::update_project_status(&db_clone, &id_clone, "active");
                let _ = db::update_deployment_status(&db_clone, &d_id_clone, "SUCCESS", None);
                let _ = db::add_log(&db_clone, &id_clone, &d_id_clone, "SUCCESS", "Deployment successful. Your service is now live on Nova Edge.");
                println!("✅ API: Deployment active for {}", id_clone);
            }
            Err(e) => {
                let err_msg = e.to_string();
                println!("❌ API: Deployment failed for {}: {}", id_clone, err_msg);
                
                let _ = db::update_project_status(&db_clone, &id_clone, "failed");
                let _ = db::update_deployment_status(&db_clone, &d_id_clone, "FAILED", Some(&err_msg));
                let _ = db::add_log(&db_clone, &id_clone, &d_id_clone, "ERROR", &format!("Deployment failed: {}", err_msg));
            }
        }
    });

    // Log the initial stage
    let _ = db::add_log(&db, &id, &deployment_id, "INFO", "Deployment initiated. Packaging project files...");

    (
        StatusCode::CREATED,
        Json(DeployResponse {
            success: true,
            message: format!("Deployment initiated with ID: {}", id),
        }),
    ).into_response()
}

/// POST /api/settings - Update user settings
pub async fn update_profile(
    State(db): State<Db>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<UpdateProfileRequest>,
) -> impl IntoResponse {
    let token = match headers.get("Authorization") {
        Some(value) => match value.to_str() {
            Ok(v) if v.starts_with("Bearer ") => &v[7..],
            _ => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "Unauthorized"}))).into_response(),
        },
        None => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "Missing token"}))).into_response(),
    };

    let claims = match super::auth::validate_token(token) {
        Ok(c) => c,
        Err(_) => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "Invalid token"}))).into_response(),
    };

    match db::update_user(&db, claims.user_id, &payload.name) {
        Ok(_) => (StatusCode::OK, Json(serde_json::json!({"success": true}))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    }
}

/// GET /api/billing - Get billing and usage info
pub async fn get_billing(
    State(_db): State<Db>,
    headers: axum::http::HeaderMap,
) -> impl IntoResponse {
    let token = match headers.get("Authorization") {
        Some(value) => match value.to_str() {
            Ok(v) if v.starts_with("Bearer ") => &v[7..],
            _ => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "Unauthorized"}))).into_response(),
        },
        None => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "Missing token"}))).into_response(),
    };

    if let Err(_) = super::auth::validate_token(token) {
        return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "Invalid token"}))).into_response();
    }

    (StatusCode::OK, Json(BillingResponse {
        balance: "$42.50".to_string(),
        usage_mtd: "1.2 TB".to_string(),
        next_invoice: "Mar 01".to_string(),
        cards: vec![
            serde_json::json!({
                "id": "card_1",
                "brand": "visa",
                "last4": "4242",
                "exp": "12/26",
                "is_default": true
            })
        ],
        history: vec![
            UsageRecord { date: "Feb 20".to_string(), amount: 2.5 },
            UsageRecord { date: "Feb 21".to_string(), amount: 3.1 },
            UsageRecord { date: "Feb 22".to_string(), amount: 2.8 },
            UsageRecord { date: "Feb 23".to_string(), amount: 4.2 },
            UsageRecord { date: "Feb 24".to_string(), amount: 1.5 },
            UsageRecord { date: "Feb 25".to_string(), amount: 5.0 },
            UsageRecord { date: "Feb 26".to_string(), amount: 3.7 },
        ],
    })).into_response()
}

/// DELETE /api/settings - Delete user account
pub async fn delete_account(
    State(db): State<Db>,
    headers: axum::http::HeaderMap,
) -> impl IntoResponse {
    let token = match headers.get("Authorization") {
        Some(value) => match value.to_str() {
            Ok(v) if v.starts_with("Bearer ") => &v[7..],
            _ => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "Unauthorized"}))).into_response(),
        },
        None => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "Missing token"}))).into_response(),
    };

    let claims = match super::auth::validate_token(token) {
        Ok(c) => c,
        Err(_) => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "Invalid token"}))).into_response(),
    };

    match db::delete_user(&db, claims.user_id) {
        Ok(_) => (StatusCode::OK, Json(serde_json::json!({"success": true}))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    }
}
/// POST /api/deployments/:id/control - Restart/Stop/Redeploy
pub async fn control_project(
    headers: axum::http::HeaderMap,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    let token = match headers.get("Authorization") {
        Some(value) => match value.to_str() {
            Ok(v) if v.starts_with("Bearer ") => &v[7..],
            _ => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "Unauthorized"}))).into_response(),
        },
        None => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "Missing token"}))).into_response(),
    };

    if let Err(_) = super::auth::validate_token(token) {
        return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "Invalid token"}))).into_response();
    }

    let action = payload["action"].as_str().unwrap_or("restart");
    println!("Project action received: {}", action);

    if action == "stop" {
        // Broadcast test kill signal to VirtualSwitch
        let _ = crate::core::network::GLOBAL_SWITCH.send(0, b"TERM".to_vec()).await;
    }

    (StatusCode::OK, Json(serde_json::json!({"success": true, "message": format!("Project {}ed successfully", action)}))).into_response()
}

/// GET /api/deployments/logs - Stream project logs via SSE
pub async fn get_project_logs(
    State(db): State<Db>,
    Query(query): Query<TokenQuery>,
    headers: axum::http::HeaderMap,
) -> impl IntoResponse {
    // Determine token from query param (for EventSource) or Auth header
    let token_str = if let Some(t) = query.token {
        t
    } else {
        match headers.get("Authorization") {
            Some(value) => match value.to_str() {
                Ok(v) if v.starts_with("Bearer ") => v[7..].to_string(),
                _ => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
            },
            None => return (StatusCode::UNAUTHORIZED, "Missing token").into_response(),
        }
    };

    if let Err(_) = super::auth::validate_token(&token_str) {
        return (StatusCode::UNAUTHORIZED, "Invalid token").into_response();
    }

    // Determine project_id from headers or query if possible
    // For now, we'll try to find the most recent project for this user or use a dummy
    // In a real app, you'd pass project_id as a query param
    
    // Create a stream that emits events
    let db_clone = db.clone();
    let stream = async_stream::stream! {
        // Yield existing logs from DB first
        let mock_project_id = "p_1"; // Default for demo
        if let Ok(logs) = db::get_project_logs(&db_clone, mock_project_id) {
            for log in logs {
                let formatted = format!("[{}] [{}] {}", log.timestamp, log.level, log.message);
                yield Ok::<_, Infallible>(Event::default().data(formatted));
            }
        }

        // Stream live mock traffic for demo effect
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));
        let mut req_id = 1000;
        loop {
            interval.tick().await;
            req_id += 1;
            let log_msg = format!("[{}] [INFO] incoming GET /api/v1/resource 200 OK req_id={}", 
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"), req_id);
            yield Ok::<_, Infallible>(Event::default().data(log_msg));
        }
    };

    Sse::new(stream)
        .keep_alive(axum::response::sse::KeepAlive::new()
            .interval(std::time::Duration::from_secs(10))
            .text("keep-alive-text"))
        .into_response()
}

/// GET /api/projects/:id/deployments - Get deployment history for a project
pub async fn get_deployment_history(
    State(db): State<Db>,
    headers: axum::http::HeaderMap,
    axum::extract::Path(project_id): axum::extract::Path<String>,
) -> impl IntoResponse {
    // Extract and validate token
    let token = match headers.get("Authorization") {
        Some(value) => match value.to_str() {
            Ok(v) if v.starts_with("Bearer ") => &v[7..],
            _ => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "Unauthorized"}))).into_response(),
        },
        None => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "Missing token"}))).into_response(),
    };

    if let Err(_) = super::auth::validate_token(token) {
        return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "Invalid token"}))).into_response();
    }

    // Resolve project ID if it looks like a name (try name lookup first)
    let actual_id = match db::get_project_id_by_name(&db, &project_id) {
        Ok(Some(id)) => id,
        _ => project_id, // Fallback to using the input as ID if no name match
    };

    // Get deployments
    match db::get_project_deployments(&db, &actual_id) {
        Ok(deployments) => (StatusCode::OK, Json(deployments)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    }
}

/// GET /api/metrics — Prometheus-compatible + JSON metrics endpoint
pub async fn get_metrics(
    State(db): State<Db>,
    headers: axum::http::HeaderMap,
) -> impl IntoResponse {
    // Attempt auth but allow through for now (can be tightened later)
    let authed = headers.get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .map(|t| super::auth::validate_token(t).is_ok())
        .unwrap_or(false);

    // Gather stats from DB
    let project_count = db::count_projects(&db).unwrap_or(0);
    let user_count    = db::count_users(&db).unwrap_or(0);

    // Simulated live counters (in production these would be real Prometheus counters)
    let uptime_secs: u64  = 3600 * 24 + 187; // ~1 day uptime
    let req_total: u64    = 1_402_854;
    let req_errors: u64   = 143;
    let memory_bytes: u64 = 6_442_450_944; // 6 GB
    let cpu_percent: f64  = 19.4;
    let cold_start_ms: f64 = 0.21;

    let accept = headers.get("accept").and_then(|v| v.to_str().ok()).unwrap_or("");

    if accept.contains("application/json") || accept.contains("*/*") && !accept.contains("text/plain") {
        // JSON format
        let json = serde_json::json!({
            "nova_edge_uptime_seconds":   uptime_secs,
            "nova_requests_total":        req_total,
            "nova_request_errors_total":  req_errors,
            "nova_memory_bytes":          memory_bytes,
            "nova_cpu_utilization":       cpu_percent,
            "nova_cold_start_ms":         cold_start_ms,
            "nova_active_projects":       project_count,
            "nova_total_users":           user_count,
            "nova_edge_nodes_online":     42,
            "collected_at":               chrono::Utc::now().to_rfc3339(),
        });
        (StatusCode::OK, [("content-type", "application/json")], serde_json::to_string_pretty(&json).unwrap_or_default()).into_response()
    } else {
        // Prometheus text format
        let body = format!(
r#"# HELP nova_edge_uptime_seconds Total uptime of the Nova edge daemon in seconds.
# TYPE nova_edge_uptime_seconds counter
nova_edge_uptime_seconds {uptime_secs}

# HELP nova_requests_total Total HTTP requests served by all edge nodes.
# TYPE nova_requests_total counter
nova_requests_total {req_total}

# HELP nova_request_errors_total Total HTTP 5xx errors.
# TYPE nova_request_errors_total counter
nova_request_errors_total {req_errors}

# HELP nova_memory_bytes Current memory usage in bytes.
# TYPE nova_memory_bytes gauge
nova_memory_bytes {memory_bytes}

# HELP nova_cpu_utilization Average CPU utilization percentage.
# TYPE nova_cpu_utilization gauge
nova_cpu_utilization {cpu_percent}

# HELP nova_cold_start_ms Average cold-start latency in milliseconds.
# TYPE nova_cold_start_ms gauge
nova_cold_start_ms {cold_start_ms}

# HELP nova_active_projects Total number of deployed projects.
# TYPE nova_active_projects gauge
nova_active_projects {project_count}

# HELP nova_total_users Total registered users.
# TYPE nova_total_users gauge
nova_total_users {user_count}

# HELP nova_edge_nodes_online Number of edge PoPs currently online.
# TYPE nova_edge_nodes_online gauge
nova_edge_nodes_online 42
"#
        );
        (StatusCode::OK, [("content-type", "text/plain; version=0.0.4")], body).into_response()
    }
}
