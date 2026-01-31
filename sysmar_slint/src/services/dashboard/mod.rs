use crate::db::PoolBanco;
use crate::queries::dashboard::generic;

pub struct DashboardResumo {
    pub total_clientes: i64,
    pub clientes_em_dia: i64,
    pub clientes_atrasados: i64,
    pub plano_diario: i64,
    pub plano_mensal: i64,
    pub plano_trimestral: i64,
    pub plano_semestral: i64,
    pub plano_anual: i64,
}

pub fn carregar_dashboard(
    pool: &PoolBanco,
) -> Result<DashboardResumo, diesel::result::Error> {
    Ok(DashboardResumo {
        total_clientes: generic::total_clientes(pool)?,
        clientes_em_dia: generic::clientes_em_dia(pool)?,
        clientes_atrasados: generic::clientes_atrasados(pool)?,
        plano_diario: generic::clientes_plano_diario(pool)?,
        plano_mensal: generic::clientes_plano_mensal(pool)?,
        plano_trimestral: generic::clientes_plano_trimestral(pool)?,
        plano_semestral: generic::clientes_plano_semestral(pool)?,
        plano_anual: generic::clientes_plano_anual(pool)?,
    })
}
