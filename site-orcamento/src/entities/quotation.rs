use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "orcamentos")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub cliente_id: i32,
    pub data_criacao: DateTimeLocal,
    pub valor_total: f64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::client::Entity",
        from = "Column::ClienteId",
        to = "super::client::Column::Id"
    )]
    Client,
    #[sea_orm(has_many = "super::quotation_item::Entity")]
    QuotationItem,
}

impl Related<super::client::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Client.def()
    }
}

impl Related<super::quotation_item::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::QuotationItem.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
