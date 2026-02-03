use chrono::Local;
use diesel::dsl::count_star;
use diesel::prelude::*;
use diesel::dsl::sql;
use diesel::sql_types::Bool;
use crate::db::PoolBanco;
use crate::schema::clientes::dsl::*;

  //========================================================================//
 //                          contagem de clientes                          //
//========================================================================//

pub fn total_clientes(
    pool: &PoolBanco,
) -> Result<i64, diesel::result::Error> {
    let mut conexao = pool.get().expect("Erro ao obter conexão");

    clientes.count().get_result(&mut conexao)
}

  //========================================================================//
 //                       clientes vigentes (em dia)                       //
//========================================================================//

pub fn clientes_em_dia(
    pool: &PoolBanco,
) -> Result<i64, diesel::result::Error> {
    let mut conexao = pool.get().expect("Erro ao obter conexão");

    let hoje = Local::now().naive_local().date();

    clientes
        .filter(vencimento.ge(hoje))
        .select(count_star())
        .get_result(&mut conexao)
}
  //========================================================================//
 //                       clientes em atraso                               //
//========================================================================//


pub fn clientes_atrasados(pool: &PoolBanco,
) -> Result<i64, diesel::result::Error> {
    
    let mut conexao = pool.get().expect("Erro ao obter conexão");

    let hoje = Local::now().naive_local().date();

    clientes
        .filter(vencimento.lt(hoje))
        .select(count_star())
        .get_result(&mut conexao)
}

  //========================================================================//
 //                       clientes por plano                               //
//========================================================================//

//////////////////////// diario ////////////////////////////////////////////
pub fn clientes_plano_diario(
    pool: &PoolBanco,
) -> Result<i64, diesel::result::Error> {
    let mut conexao = pool.get().expect("Erro ao obter conexão");

    clientes
        .filter(plano.is_not_null())
        .filter(sql::<Bool>("plano = 'diaria'"))
        .select(count_star())
        .get_result(&mut conexao)
}
//////////////////////// mensal ////////////////////////////////////////////
pub fn clientes_plano_mensal(
    pool: &PoolBanco,
) -> Result<i64, diesel::result::Error> {
    let mut conexao = pool.get().expect("Erro ao obter conexão");

    clientes
        .filter(plano.is_not_null())
        .filter(sql::<Bool>("plano = 'mensal'"))
        .select(count_star())
        .get_result(&mut conexao)
}
//////////////////////// trimestral ////////////////////////////////////////
pub fn clientes_plano_trimestral(
    pool: &PoolBanco,
) -> Result<i64, diesel::result::Error>{

    let mut conexao = pool.get().expect("Erro ao obter conexão");

    clientes
        .filter(plano.is_not_null())
        .filter(sql::<Bool>("plano = 'trimestral'"))
        .select(count_star())
        .get_result(&mut conexao)
}

//////////////////////// semestral /////////////////////////////////////////
pub fn clientes_plano_semestral(
    pool: &PoolBanco,
) -> Result<i64, diesel::result::Error> {
    let mut conexao = pool.get().expect("Erro ao obter conexão");

    clientes
        .filter(plano.is_not_null())
        .filter(sql::<Bool>("plano = 'semestral'"))
        .select(count_star())
        .get_result(&mut conexao)
}
//////////////////////// anual /////////////////////////////////////////////
pub fn clientes_plano_anual(
    pool: &PoolBanco,
) -> Result<i64, diesel::result::Error> {
    let mut conexao = pool.get().expect("Erro ao obter conexão");

    clientes
        .filter(plano.is_not_null())
        .filter(sql::<Bool>("plano = 'anual'"))
        .select(count_star())
        .get_result(&mut conexao)
}
