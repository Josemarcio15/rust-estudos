use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "clientes")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub nome: String,
    pub email: String,
    pub telefone: String,
    pub endereco: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::quotation::Entity")]
    Quotation,
}

impl Related<super::quotation::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Quotation.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
