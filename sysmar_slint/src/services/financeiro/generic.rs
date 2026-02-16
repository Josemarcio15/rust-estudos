use crate::db::PoolBanco;
use crate::queries::financeiro::generic::*;
use crate::models::financeiro::financeiro::ResumoFinanceiro;
use chrono::NaiveDate;

pub fn get_resumo_financeiro(
    pool: &PoolBanco,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> ResumoFinanceiro {
    let diario = soma_pagamentos_por_periodo(pool, start_date, end_date, "diaria").unwrap_or(0.0);
    let mensal = soma_pagamentos_por_periodo(pool, start_date, end_date, "mensal").unwrap_or(0.0);
    let trimestral = soma_pagamentos_por_periodo(pool, start_date, end_date, "trimestral").unwrap_or(0.0);
    let semestral = soma_pagamentos_por_periodo(pool, start_date, end_date, "semestral").unwrap_or(0.0);
    let anual = soma_pagamentos_por_periodo(pool, start_date, end_date, "anual").unwrap_or(0.0);
    
    let total = diario + mensal + trimestral + semestral + anual;

    ResumoFinanceiro {
        diario,
        mensal,
        trimestral,
        semestral,
        anual,
        total,
    }
}
