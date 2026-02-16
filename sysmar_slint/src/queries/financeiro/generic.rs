use diesel::prelude::*;
use diesel::dsl::sql;
use diesel::sql_types::Bool;
use crate::db::PoolBanco;
use crate::schema::pagamentos::dsl::*;
use chrono::NaiveDate;

pub fn soma_pagamentos_por_periodo(
    pool: &PoolBanco,
    start_date: NaiveDate,
    end_date: NaiveDate,
    target_plano: &str,
) -> Result<f64, diesel::result::Error> {
    let mut conexao = pool.get().expect("Erro ao obter conexão");

    let total: Option<bigdecimal::BigDecimal> = pagamentos
        .filter(data_pagamento.ge(start_date))
        .filter(data_pagamento.le(end_date))
        .filter(sql::<Bool>(&format!("plano = '{}'", target_plano)))
        .select(diesel::dsl::sum(valor_pago))
        .get_result(&mut conexao)?;

    Ok(total.map(|v: bigdecimal::BigDecimal| {
        use bigdecimal::ToPrimitive;
        v.to_f64().unwrap_or(0.0)
    }).unwrap_or(0.0))

}
