use chrono::NaiveDate;
use sea_orm::DatabaseConnection;
use num_traits::ToPrimitive;
use crate::db::PoolBanco;
use crate::entities::pagamentos;
use crate::queries::dashboard::generic;
use crate::entities::Plano;

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
pub struct DashboardUI {
    pub total_clientes: i32,
    pub clientes_em_dia: i32,
    pub clientes_atrasados: i32,

    pub percentual_clientes_em_dia: f32,
    pub percentual_clientes_atrasados: f32,
    pub percentual_plano_diario: f32,
    pub percentual_plano_mensal: f32,
    pub percentual_plano_trimestral: f32,
    pub percentual_plano_semestral: f32,
    pub percentual_plano_anual: f32,

    pub plano_diario: i32,
    pub plano_mensal: i32,
    pub plano_trimestral: i32,
    pub plano_semestral: i32,
    pub plano_anual: i32,
}

#[derive(Default)]
pub struct ValoresObtidos {
    pub total_diario: i64,
    pub total_mensal: i64,
    pub total_trimestral: i64,
    pub total_semestral: i64,
    pub total_anual: i64,
    pub total_geral: i64,
}

pub async fn carregar_dashboard(
    pool: &PoolBanco,
) -> Result<DashboardResumo, sea_orm::DbErr> {
    Ok(DashboardResumo {
        total_clientes: generic::total_clientes(pool).await? as i64,
        clientes_em_dia: generic::clientes_em_dia(pool).await? as i64,
        clientes_atrasados: generic::clientes_atrasados(pool).await? as i64,
        plano_diario: generic::clientes_plano_diario(pool).await? as i64,
        plano_mensal: generic::clientes_plano_mensal(pool).await? as i64,
        plano_trimestral: generic::clientes_plano_trimestral(pool).await? as i64,
        plano_semestral: generic::clientes_plano_semestral(pool).await? as i64,
        plano_anual: generic::clientes_plano_anual(pool).await? as i64,
    })
}

pub async fn carregar_dashboard_ui(
    pool: &PoolBanco,
) -> Result<DashboardUI, sea_orm::DbErr> {
    let resumo = carregar_dashboard(pool).await?;
    
    let total_clientes = resumo.total_clientes as i32;
    let clientes_em_dia= resumo.clientes_em_dia as i32;
    let clientes_atrasados= resumo.clientes_atrasados as i32;
    let plano_diario= resumo.plano_diario as i32;
    let plano_mensal= resumo.plano_mensal as i32;
    let plano_trimestral= resumo.plano_trimestral as i32;
    let plano_semestral= resumo.plano_semestral as i32;
    let plano_anual= resumo.plano_anual as i32;

    let percentual_clientes_em_dia = porcentagem(clientes_em_dia, total_clientes);
    let percentual_clientes_atrasados = porcentagem(clientes_atrasados, total_clientes);
    let percentual_plano_diario = porcentagem(plano_diario, total_clientes);
    let percentual_plano_mensal = porcentagem(plano_mensal, total_clientes);
    let percentual_plano_trimestral = porcentagem(plano_trimestral, total_clientes);
    let percentual_plano_semestral = porcentagem(plano_semestral, total_clientes);
    let percentual_plano_anual = porcentagem(plano_anual, total_clientes);

    Ok(DashboardUI {
        total_clientes,
        clientes_em_dia,
        clientes_atrasados,
        plano_diario,
        plano_mensal,
        plano_trimestral,
        plano_semestral,
        plano_anual,

        percentual_clientes_em_dia,
        percentual_clientes_atrasados,
        percentual_plano_diario,
        percentual_plano_mensal,
        percentual_plano_trimestral,
        percentual_plano_semestral,
        percentual_plano_anual,
    })
}

pub fn porcentagem(parte: i32, total: i32) -> f32 {
    if total <= 0 {
        return 0.0;
    }

    (parte as f32 * 100.0) / total as f32
}

pub async fn calcular_valores(db: &DatabaseConnection, inicio: &str, fim: &str) -> Result<ValoresObtidos, sea_orm::DbErr> {
    let data_inicio = NaiveDate::parse_from_str(inicio, "%Y-%m-%d").unwrap();
    let data_fim = NaiveDate::parse_from_str(fim, "%Y-%m-%d").unwrap();

    let dados_brutos = generic::filtrar_pagamentos_por_data(db, data_inicio, data_fim).await?;
    
    Ok(organizar_pagamentos(&dados_brutos))
}

pub fn organizar_pagamentos(dados: &Vec<pagamentos::Model>) -> ValoresObtidos {
    let mut resumo = ValoresObtidos::default();

    for p in dados {
        let centavos = (p.valor_pago * sea_orm::prelude::Decimal::from(100))
            .to_i64()
            .unwrap_or(0);

        match p.plano {
            Plano::Diaria => resumo.total_diario += centavos,
            Plano::Mensal => resumo.total_mensal += centavos,
            Plano::Trimestral => resumo.total_trimestral += centavos,
            Plano::Semestral => resumo.total_semestral += centavos,
            Plano::Anual => resumo.total_anual += centavos,
            Plano::Nenhum => (),
        }

        resumo.total_geral += centavos;
    }
    resumo
}