use sea_orm::*;
use chrono::Local;
// Supondo que você gerou as entidades com 'sea-orm-cli generate entity'
// Ajuste o caminho conforme a estrutura do seu projeto (ex: crate::entities::clientes)
use crate::entities::clientes; 
use crate::entities::pagamentos;
use chrono::NaiveDate;

  //========================================================================//
 //                          contagem de clientes                          //
//========================================================================//

pub async fn total_clientes(db: &DatabaseConnection) -> Result<u64, DbErr> {
    // No SeaORM, não precisamos pegar conexão do pool manualmente,
    // o DatabaseConnection já gerencia isso internamente.
    clientes::Entity::find().count(db).await
}

  //========================================================================//
 //                       clientes vigentes (em dia)                       //
//========================================================================//

pub async fn clientes_em_dia(db: &DatabaseConnection) -> Result<u64, DbErr> {
    let hoje = Local::now().naive_local().date();

    // gte = greater than or equal (maior ou igual)
    clientes::Entity::find()
        .filter(clientes::Column::Vencimento.gte(hoje))
        .count(db)
        .await
}

  //========================================================================//
 //                       clientes em atraso                               //
//========================================================================//

pub async fn clientes_atrasados(db: &DatabaseConnection) -> Result<u64, DbErr> {
    let hoje = Local::now().naive_local().date();

    // lt = less than (menor que)
    clientes::Entity::find()
        .filter(clientes::Column::Vencimento.lt(hoje))
        .count(db)
        .await
}

  //========================================================================//
 //                       clientes por plano                               //
//========================================================================//

// Dica: No SeaORM, verificar igualdade (.eq) já implica que não é NULL,
// então não precisamos do filtro explícito "is_not_null" se buscamos um valor específico.

//////////////////////// diario ////////////////////////////////////////////
pub async fn clientes_plano_diario(db: &DatabaseConnection) -> Result<u64, DbErr> {
    clientes::Entity::find()
        .filter(clientes::Column::Plano.eq("diaria"))
        .count(db)
        .await
}

//////////////////////// mensal ////////////////////////////////////////////
pub async fn clientes_plano_mensal(db: &DatabaseConnection) -> Result<u64, DbErr> {
    clientes::Entity::find()
        .filter(clientes::Column::Plano.eq("mensal"))
        .count(db)
        .await
}

//////////////////////// trimestral ////////////////////////////////////////
pub async fn clientes_plano_trimestral(db: &DatabaseConnection) -> Result<u64, DbErr> {
    clientes::Entity::find()
        .filter(clientes::Column::Plano.eq("trimestral"))
        .count(db)
        .await
}

//////////////////////// semestral /////////////////////////////////////////
pub async fn clientes_plano_semestral(db: &DatabaseConnection) -> Result<u64, DbErr> {
    clientes::Entity::find()
        .filter(clientes::Column::Plano.eq("semestral"))
        .count(db)
        .await
}

//////////////////////// anual /////////////////////////////////////////////
pub async fn clientes_plano_anual(db: &DatabaseConnection) -> Result<u64, DbErr> {
    clientes::Entity::find()
        .filter(clientes::Column::Plano.eq("anual"))
        .count(db)
        .await
}

pub async fn filtrar_pagamentos_por_data(db: &DatabaseConnection, inicio_data: NaiveDate, fim_data: NaiveDate) -> Result<Vec<pagamentos::Model>, DbErr> {
 pagamentos::Entity::find()
    .filter(pagamentos::Column::DataPagamento.between(inicio_data, fim_data))
    .all(db)
    .await   
}
