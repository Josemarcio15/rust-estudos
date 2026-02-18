use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "tags")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub cliente_id: i32,
    #[sea_orm(unique)]
    pub codigo_tag: String,
    pub data_cadastro: Option<Date>,
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
