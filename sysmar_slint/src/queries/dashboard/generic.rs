use diesel::prelude::*;
use diesel::dsl::{sql, count_distinct};
use diesel::sql_types::Bool;
use crate::db::PoolBanco;
use crate::schema::clientes::dsl as cl_dsl;
use crate::schema::pagamentos::dsl as pg_dsl;
use chrono::{NaiveDate, Local};

  //========================================================================//
 //                          contagem de clientes                          //
//========================================================================//

pub fn total_clientes(
    pool: &PoolBanco,
) -> Result<i64, diesel::result::Error> {
    let mut conexao = pool.get().expect("Erro ao obter conexão");

    cl_dsl::clientes.count().get_result(&mut conexao)
}

  //========================================================================//
 //                       clientes vigentes (em dia)                       //
//========================================================================//

pub fn clientes_em_dia(
    pool: &PoolBanco,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<i64, diesel::result::Error> {
    let mut conexao = pool.get().expect("Erro ao obter conexão");
    let hoje = Local::now().naive_local().date();

    cl_dsl::clientes
        .inner_join(pg_dsl::pagamentos.on(pg_dsl::cliente_id.eq(cl_dsl::id)))
        .filter(pg_dsl::data_pagamento.ge(start_date))
        .filter(pg_dsl::data_pagamento.le(end_date))
        .filter(cl_dsl::vencimento.ge(hoje))
        .select(count_distinct(cl_dsl::id))
        .get_result(&mut conexao)
}
  //========================================================================//
 //                       clientes em atraso                               //
//========================================================================//


pub fn clientes_atrasados(
    pool: &PoolBanco,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<i64, diesel::result::Error> {
    let mut conexao = pool.get().expect("Erro ao obter conexão");
    let hoje = Local::now().naive_local().date();

    cl_dsl::clientes
        .inner_join(pg_dsl::pagamentos.on(pg_dsl::cliente_id.eq(cl_dsl::id)))
        .filter(pg_dsl::data_pagamento.ge(start_date))
        .filter(pg_dsl::data_pagamento.le(end_date))
        .filter(cl_dsl::vencimento.lt(hoje))
        .select(count_distinct(cl_dsl::id))
        .get_result(&mut conexao)
}

  //========================================================================//
 //                       clientes por plano                               //
//========================================================================//

pub fn clientes_plano_diario(
    pool: &PoolBanco,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<i64, diesel::result::Error> {
    let mut conexao = pool.get().expect("Erro ao obter conexão");

    cl_dsl::clientes
        .inner_join(pg_dsl::pagamentos.on(pg_dsl::cliente_id.eq(cl_dsl::id)))
        .filter(pg_dsl::data_pagamento.ge(start_date))
        .filter(pg_dsl::data_pagamento.le(end_date))
        .filter(sql::<Bool>("pagamentos.plano = 'diaria'"))
        .select(count_distinct(cl_dsl::id))
        .get_result(&mut conexao)
}
//////////////////////// mensal ////////////////////////////////////////////
pub fn clientes_plano_mensal(
    pool: &PoolBanco,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<i64, diesel::result::Error> {
    let mut conexao = pool.get().expect("Erro ao obter conexão");

    cl_dsl::clientes
        .inner_join(pg_dsl::pagamentos.on(pg_dsl::cliente_id.eq(cl_dsl::id)))
        .filter(pg_dsl::data_pagamento.ge(start_date))
        .filter(pg_dsl::data_pagamento.le(end_date))
        .filter(sql::<Bool>("pagamentos.plano = 'mensal'"))
        .select(count_distinct(cl_dsl::id))
        .get_result(&mut conexao)
}
//////////////////////// trimestral ////////////////////////////////////////
pub fn clientes_plano_trimestral(
    pool: &PoolBanco,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<i64, diesel::result::Error>{
    let mut conexao = pool.get().expect("Erro ao obter conexão");

    cl_dsl::clientes
        .inner_join(pg_dsl::pagamentos.on(pg_dsl::cliente_id.eq(cl_dsl::id)))
        .filter(pg_dsl::data_pagamento.ge(start_date))
        .filter(pg_dsl::data_pagamento.le(end_date))
        .filter(sql::<Bool>("pagamentos.plano = 'trimestral'"))
        .select(count_distinct(cl_dsl::id))
        .get_result(&mut conexao)
}

//////////////////////// semestral /////////////////////////////////////////
pub fn clientes_plano_semestral(
    pool: &PoolBanco,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<i64, diesel::result::Error> {
    let mut conexao = pool.get().expect("Erro ao obter conexão");

    cl_dsl::clientes
        .inner_join(pg_dsl::pagamentos.on(pg_dsl::cliente_id.eq(cl_dsl::id)))
        .filter(pg_dsl::data_pagamento.ge(start_date))
        .filter(pg_dsl::data_pagamento.le(end_date))
        .filter(sql::<Bool>("pagamentos.plano = 'semestral'"))
        .select(count_distinct(cl_dsl::id))
        .get_result(&mut conexao)
}
//////////////////////// anual /////////////////////////////////////////////
pub fn clientes_plano_anual(
    pool: &PoolBanco,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<i64, diesel::result::Error> {
    let mut conexao = pool.get().expect("Erro ao obter conexão");

    cl_dsl::clientes
        .inner_join(pg_dsl::pagamentos.on(pg_dsl::cliente_id.eq(cl_dsl::id)))
        .filter(pg_dsl::data_pagamento.ge(start_date))
        .filter(pg_dsl::data_pagamento.le(end_date))
        .filter(sql::<Bool>("pagamentos.plano = 'anual'"))
        .select(count_distinct(cl_dsl::id))
        .get_result(&mut conexao)
}
