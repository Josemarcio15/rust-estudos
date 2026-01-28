use diesel::prelude::*;
use crate::db::connection::get_connection;

pub fn total_clientes() -> u64 {
    let mut conn = get_connection();

    let resultado: Option<u64> =
        conn.query_first("SELECT COUNT(*) FROM clientes")
            .expect("Erro ao contar clientes");

    resultado.unwrap_or(0)
}