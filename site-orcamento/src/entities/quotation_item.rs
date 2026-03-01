use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "itens_orcamento")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub orcamento_id: i32,
    pub produto_id: i32,
    pub quantidade: i32,
    pub valor_unitario: f64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::quotation::Entity",
        from = "Column::OrcamentoId",
        to = "super::quotation::Column::Id"
    )]
    Quotation,
    #[sea_orm(
        belongs_to = "super::product::Entity",
        from = "Column::ProdutoId",
        to = "super::product::Column::Id"
    )]
    Product,
}

impl Related<super::quotation::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Quotation.def()
    }
}

impl Related<super::product::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Product.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
