pub mod prelude;

pub mod clientes;
pub mod fluxo_catraca;
pub mod pagamentos;
pub mod tags;

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "sexo")]
pub enum Sexo {
    #[sea_orm(string_value = "M")]
    M,
    #[sea_orm(string_value = "F")]
    F,
    #[sea_orm(string_value = "O")]
    O,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "plano")]
pub enum Plano {
    #[sea_orm(string_value = "DIARIA")]
    Diaria,
    #[sea_orm(string_value = "MENSAL")]
    Mensal,
    #[sea_orm(string_value = "TRIMESTRAL")]
    Trimestral,
    #[sea_orm(string_value = "SEMESTRAL")]
    Semestral,
    #[sea_orm(string_value = "ANUAL")]
    Anual,
    #[sea_orm(string_value = "NENHUM")]
    Nenhum,
}
