// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(mysql_type(name = "Enum"))]
    pub struct ClientesPlanoEnum;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(mysql_type(name = "Enum"))]
    pub struct ClientesSexoEnum;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(mysql_type(name = "Enum"))]
    pub struct FluxoCatracaPlanoEnum;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(mysql_type(name = "Enum"))]
    pub struct PagamentosPlanoEnum;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(mysql_type(name = "Enum"))]
    pub struct TaxasTipoEnum;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ClientesSexoEnum;
    use super::sql_types::ClientesPlanoEnum;

    clientes (id) {
        id -> Integer,
        #[max_length = 255]
        nome -> Varchar,
        #[max_length = 15]
        cpf -> Varchar,
        #[max_length = 1]
        sexo -> Nullable<ClientesSexoEnum>,
        #[max_length = 255]
        endereco -> Nullable<Varchar>,
        numero -> Nullable<Integer>,
        #[max_length = 255]
        bairro -> Nullable<Varchar>,
        data_nascimento -> Nullable<Date>,
        #[max_length = 255]
        email -> Nullable<Varchar>,
        #[max_length = 20]
        telefone -> Nullable<Varchar>,
        complemento -> Nullable<Text>,
        #[max_length = 10]
        plano -> Nullable<ClientesPlanoEnum>,
        dia_do_pagamento -> Tinyint,
        vencimento -> Nullable<Date>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::FluxoCatracaPlanoEnum;

    fluxo_catraca (id) {
        id -> Integer,
        cliente_id -> Integer,
        hora_entrada -> Time,
        #[max_length = 10]
        plano -> Nullable<FluxoCatracaPlanoEnum>,
        dia_entrada -> Date,
        #[max_length = 255]
        status -> Nullable<Varchar>,
        entrada_duplicada -> Bool,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::PagamentosPlanoEnum;

    pagamentos (id) {
        id -> Integer,
        cliente_id -> Integer,
        valor_pago -> Decimal,
        data_pagamento -> Date,
        mes_referente -> Integer,
        #[max_length = 10]
        plano -> PagamentosPlanoEnum,
        observacoes -> Nullable<Text>,
    }
}

diesel::table! {
    tags (id) {
        id -> Integer,
        cliente_id -> Integer,
        #[max_length = 100]
        codigo_tag -> Varchar,
        data_cadastro -> Nullable<Date>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::TaxasTipoEnum;

    taxas (id) {
        id -> Integer,
        #[max_length = 10]
        tipo -> TaxasTipoEnum,
        valor -> Decimal,
        data_inicio -> Date,
        data_fim -> Nullable<Date>,
    }
}

diesel::joinable!(fluxo_catraca -> clientes (cliente_id));
diesel::joinable!(pagamentos -> clientes (cliente_id));
diesel::joinable!(tags -> clientes (cliente_id));

diesel::allow_tables_to_appear_in_same_query!(clientes, fluxo_catraca, pagamentos, tags, taxas,);
