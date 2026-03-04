use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::services::ServeDir;
use tower_http::cors::CorsLayer;

pub mod handlers;
pub mod db;
pub mod auth;

pub async fn start_server(port: u16) {
    // Initialize Database
    let db = db::init_db().expect("Failed to initialize database");

    // Define Routes
    let app = Router::new()
        // API Routes
        .route("/api/signup", post(handlers::signup))
        .route("/api/login", post(handlers::login))
        .route("/api/me", get(handlers::get_profile))
        .route("/api/projects", get(handlers::get_projects))
        .route("/api/deployments", post(handlers::deploy_project))
        .route("/api/stats", get(handlers::get_stats))
        .route("/api/billing", get(handlers::get_billing))
        .route("/api/settings", post(handlers::update_profile))
        .route("/api/settings", axum::routing::delete(handlers::delete_account))
        .route("/api/deployments/logs", get(handlers::get_project_logs))
        .route("/api/deployments/control", post(handlers::control_project))
        .route("/api/projects/:id/deployments", get(handlers::get_deployment_history))
        .route("/api/metrics", get(handlers::get_metrics))
        // Serve Static Files (Frontend)
        .nest_service("/", ServeDir::new("www"))
        .layer(CorsLayer::permissive())
        .with_state(db);

    // Start Server
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("🚀 Nova Cloud Dashboard running at http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
