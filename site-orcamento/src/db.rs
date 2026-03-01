use sea_orm::{Database, DatabaseConnection};
use std::env;
use dotenvy::dotenv;
use tokio::sync::OnceCell;

static DB: OnceCell<DatabaseConnection> = OnceCell::const_new();

pub async fn get_db() -> &'static DatabaseConnection {
    DB.get_or_init(|| async {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        tracing::info!("Connecting to database...");
        match Database::connect(&database_url).await {
            Ok(conn) => {
                tracing::info!("Connected to database successfully");
                conn
            }
            Err(e) => {
                tracing::error!("Failed to connect to database: {:?}", e);
                panic!("Failed to connect to database: {:?}", e);
            }
        }
    }).await
}
