use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use super::Plano;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "pagamentos")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub cliente_id: i32,
    pub valor_pago: Decimal,
    pub data_pagamento: Date,
    pub mes_referente: i32,
    pub plano: Plano,
    #[sea_orm(column_type = "Text", nullable)]
    pub observacoes: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::clientes::Entity",
        from = "Column::ClienteId",
        to = "super::clientes::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Clientes,
}

impl Related<super::clientes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Clientes.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
