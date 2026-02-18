use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use super::{Sexo, Plano};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "clientes")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub nome: String,
    #[sea_orm(unique)]
    pub cpf: String,
    pub sexo: Option<Sexo>,
    pub endereco: Option<String>,
    pub numero: Option<i32>,
    pub bairro: Option<String>,
    pub data_nascimento: Option<Date>,
    pub email: Option<String>,
    pub telefone: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub complemento: Option<String>,
    pub plano: Option<Plano>,
    pub dia_do_pagamento: i8,
    pub vencimento: Option<Date>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::pagamentos::Entity")]
    Pagamentos,
    #[sea_orm(has_many = "super::tags::Entity")]
    Tags,
    #[sea_orm(has_many = "super::fluxo_catraca::Entity")]
    FluxoCatraca,
}

impl Related<super::pagamentos::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Pagamentos.def()
    }
}

impl Related<super::tags::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tags.def()
    }
}

impl Related<super::fluxo_catraca::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FluxoCatraca.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
