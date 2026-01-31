use diesel::prelude::*;
use crate::schema::clientes;
use crate::schema::sql_types::{ClientesPlanoEnum, ClientesSexoEnum};
use chrono::NaiveDate;

#[derive(Queryable)]
pub struct Cliente{
    pub id: i32,
    pub nome: String,
    pub cpf: String,
    pub sexo: Option<String>,
    pub endereco: Option<String>,
    pub numero: Option<i32>,
    pub bairro: Option<String>,
    pub data_nascimento: Option<NaiveDate>,
    pub email: Option<String>,
    pub telefone: Option<String>,
    pub complemento: Option<String>,
    pub plano: Option<String>,
    pub dia_do_pagamento: i8,
    pub vencimento: Option<NaiveDate>,
}