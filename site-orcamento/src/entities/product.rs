use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "produtos")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub codigo: String,
    pub nome: String,
    pub valor: f64,
    pub estoque: i32,
    pub categoria: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::quotation_item::Entity")]
    QuotationItem,
}

impl Related<super::quotation_item::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::QuotationItem.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
