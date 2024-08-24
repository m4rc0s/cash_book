use std::{env,fmt};
use axum::Router;
use sqlx::{postgres::PgPoolOptions, PgPool};

pub(crate) mod service;
pub(crate) mod repository;
pub(crate) mod routes;

#[derive(PartialEq)]
enum AppEnv {
    Local,
}

struct SystemSettings {
    pub server_address: String,
    pub db_url: String
}

impl SystemSettings {
    pub async fn new() -> Self {
        let db_url = std::env::var("DATABASE_URL").unwrap();
        let server_address = env::var("SERVER_ADDRESS").unwrap();

        Self {
            db_url,
            server_address
        }
    }
}

#[derive(Clone)]
pub(crate) struct AppState {
    pub(crate) db_conn_pool: PgPool
}

impl AppState {
    pub(crate) async fn new(settings: &SystemSettings) -> Self {
        let db_conn_pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&settings.db_url).await.unwrap();

        Self {
            db_conn_pool       
        }
    }
}

#[tokio::main]
async fn main() {

    let app_env = match env::var("APP_ENV") {
        _ => AppEnv::Local,
    };

    println!("Running in {app_env} mode");

    if app_env == AppEnv::Local {
        match dotenvy::dotenv() {
            Ok(path) => println!(".env read successfully from {}", path.display()),
            Err(e) => println!("Could not load .env file: {e}"),
        };
    }

    let system_settings = SystemSettings::new().await;
    let state = AppState::new(&system_settings).await;
    let app = Router::new().nest("/api/v1", routes::routes(state));
    let listener = tokio::net::TcpListener::bind(system_settings.server_address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

impl fmt::Display for AppEnv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppEnv::Local => write!(f, "local"),
        }
    }
}