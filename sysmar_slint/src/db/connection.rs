use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use std::env;

pub fn get_connection() -> MysqlConnection {
    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL não definida");

    MysqlConnection::establish(&database_url)
        .expect("Erro ao conectar no MariaDB/MySQL")
}
