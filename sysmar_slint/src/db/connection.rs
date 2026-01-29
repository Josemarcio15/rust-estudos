use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;
use std::env;

pub type PoolBanco = Pool<ConnectionManager<MysqlConnection>>;

pub fn criar_pool_banco() -> PoolBanco{dotenv().ok();
    let url_banco = 
        env::var("DATABASE_URL").expect("DATABASE_URL nao definida no .env");

    let gerenciador = 
        ConnectionManager::<MysqlConnection>::new(url_banco);

    Pool::builder()
        .build(gerenciador)
        .expect("Erro ao criar pool de conexoes")
}