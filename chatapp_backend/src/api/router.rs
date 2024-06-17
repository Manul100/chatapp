use std::sync::Arc;
use axum::{
    http::{header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE}, HeaderValue, Method}, middleware, routing::{get, post}, Router
};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tower_http::cors::CorsLayer;
use super::config::{self, Config};
use dotenvy::dotenv;
use crate::application::{
    handler::{get_me_handler, register_user_handler, login_user_handler, logout_handler},
    jwt_auth::auth};
use super::health_checker_handler;

pub struct AppState {
    pub db: Pool<Postgres>,
    pub env: Config,
}

pub async fn create_router() -> Router {
    
    dotenv().ok();
    let config = Config::init();
    let pool = create_pool(&config).await;
    let cors = create_cors();
    let app_state = Arc::new(AppState {
        db: pool.clone(),
        env: config.clone(),
    });
    
    Router::new()
        .route("/api/healthchecker", get(health_checker_handler))
        .route("/api/auth/register", post(register_user_handler))
        .route("/api/auth/login", post(login_user_handler))
        .route("/api/auth/logout", get(logout_handler)
            .route_layer(middleware::from_fn_with_state(app_state.clone(), auth)))
        .route("/api/auth/me", get(get_me_handler)
            .route_layer(middleware::from_fn_with_state(app_state.clone(), auth)))
        .with_state(app_state)
        .layer(cors)
}

pub async fn create_pool(config: &config::Config) -> Pool<Postgres> {
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };
    return pool;
}

pub fn create_cors() -> CorsLayer{
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);
    return cors;
}