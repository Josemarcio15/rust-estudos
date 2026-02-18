use sea_orm::{Database, DatabaseConnection};
use dotenvy::dotenv;
use std::env;

pub type PoolBanco = DatabaseConnection;

pub async fn criar_pool_banco() -> PoolBanco {
    dotenv().ok();
    let url_banco = 
        env::var("DATABASE_URL").expect("DATABASE_URL nao definida no .env");

    Database::connect(url_banco)
        .await
        .expect("Erro ao criar pool de conexoes")
}