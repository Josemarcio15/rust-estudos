use mysql::prelude::*;
use crate::db::connection::get_connection;
use mysql::params;
#[derive(Debug)]
pub struct Cliente {
    pub id: i32,
    pub nome: String,
    pub cpf: String,
}

pub fn obter_clientes_por_nome(nome: &str) -> Vec<Cliente> {
    let mut conn = get_connection();

    let like_nome = format!("%{}%", nome);

    let resultado = conn.exec_map(
        "SELECT id, nome, cpf FROM clientes WHERE nome LIKE :nome",
        params! {
            "nome" => like_nome
        },
        |(id, nome, cpf)| Cliente { id, nome, cpf },
    );

    match resultado {
        Ok(clientes) => clientes,
        Err(e) => {
            eprintln!("Erro ao obter clientes por nome: {}", e);
            Vec::new()
        }
    }
}
pub fn total_clientes() -> u64 {
    let mut conn = get_connection();

    let resultado: Option<u64> =
        conn.query_first("SELECT COUNT(*) FROM clientes")
            .expect("Erro ao contar clientes");

    resultado.unwrap_or(0)
}
