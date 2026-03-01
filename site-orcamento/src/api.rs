use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, QueryOrder, Set, TransactionTrait, QuerySelect, ColumnTrait, DeriveColumn, EnumIter};
use serde::{Deserialize, Serialize};
use crate::db::get_db;
use crate::entities::{self, Client, Product, Quotation, QuotationItem};

#[derive(Serialize, Deserialize)]
pub struct CreateClient {
    pub nome: String,
    pub email: String,
    pub telefone: String,
    pub endereco: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateProduct {
    pub codigo: String,
    pub nome: String,
    pub valor: f64,
    pub estoque: i32,
    pub categoria: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateQuotationItem {
    pub produto_id: i32,
    pub quantidade: i32,
    pub valor_unitario: f64,
}

#[derive(Serialize, Deserialize)]
pub struct CreateQuotation {
    pub cliente_id: i32,
    pub itens: Vec<CreateQuotationItem>,
}

pub fn app() -> Router {
    Router::new()
        .route("/api/dashboard", get(get_dashboard_data))
        .route("/api/clients", get(list_clients).post(create_client))
        .route("/api/products", get(list_products).post(create_product))
        .route("/api/quotations", get(list_quotations).post(create_quotation))
}

async fn get_dashboard_data() -> Json<(f64, i32)> {
    let db = get_db().await;
    
    // Sum calculation using select_only
    let total_value: f64 = entities::quotation::Entity::find()
        .select_only()
        .column_as(entities::quotation::Column::ValorTotal.sum(), "valor_total")
        .into_tuple::<Option<f64>>()
        .one(db)
        .await
        .unwrap_or_default()
        .flatten()
        .unwrap_or(0.0);

    let count = entities::quotation::Entity::find().count(db).await.unwrap_or(0) as i32;

    Json((total_value, count))
}

async fn list_clients() -> Json<Vec<entities::client::Model>> {
    let db = get_db().await;
    let clients = entities::client::Entity::find()
        .order_by_asc(entities::client::Column::Nome)
        .all(db)
        .await
        .unwrap_or_default();
    Json(clients)
}

async fn create_client(Json(payload): Json<CreateClient>) -> Json<entities::client::Model> {
    let db = get_db().await;
    let client = entities::client::ActiveModel {
        nome: Set(payload.nome),
        email: Set(payload.email),
        telefone: Set(payload.telefone),
        endereco: Set(payload.endereco),
        ..Default::default()
    };
    let res = client.insert(db).await.expect("Failed to insert client");
    Json(res)
}

async fn list_products() -> Json<Vec<entities::product::Model>> {
    let db = get_db().await;
    let products = entities::product::Entity::find()
        .order_by_asc(entities::product::Column::Nome)
        .all(db)
        .await
        .unwrap_or_default();
    Json(products)
}

async fn create_product(Json(payload): Json<CreateProduct>) -> Json<entities::product::Model> {
    let db = get_db().await;
    let product = entities::product::ActiveModel {
        codigo: Set(payload.codigo),
        nome: Set(payload.nome),
        valor: Set(payload.valor),
        estoque: Set(payload.estoque),
        categoria: Set(payload.categoria),
        ..Default::default()
    };
    let res = product.insert(db).await.expect("Failed to insert product");
    Json(res)
}

async fn list_quotations() -> Json<Vec<entities::quotation::Model>> {
    let db = get_db().await;
    let quotations = entities::quotation::Entity::find()
        .order_by_desc(entities::quotation::Column::DataCriacao)
        .all(db)
        .await
        .unwrap_or_default();
    Json(quotations)
}

async fn create_quotation(Json(payload): Json<CreateQuotation>) -> Json<entities::quotation::Model> {
    let db = get_db().await;
    
    let res = db.transaction::<_, entities::quotation::Model, sea_orm::DbErr>(|txn| {
        Box::pin(async move {
            let mut total_value = 0.0;
            for item in &payload.itens {
                total_value += item.valor_unitario * item.quantidade as f64;
            }

            let quotation = entities::quotation::ActiveModel {
                cliente_id: Set(payload.cliente_id),
                valor_total: Set(total_value),
                data_criacao: Set(chrono::Local::now()),
                ..Default::default()
            };
            
            let quotation_res = quotation.insert(txn).await?;

            for item in payload.itens {
                let q_item = entities::quotation_item::ActiveModel {
                    orcamento_id: Set(quotation_res.id),
                    produto_id: Set(item.produto_id),
                    quantidade: Set(item.quantidade),
                    valor_unitario: Set(item.valor_unitario),
                    ..Default::default()
                };
                q_item.insert(txn).await?;
            }

            Ok(quotation_res)
        })
    }).await.expect("Transaction failed");

    Json(res)
}
