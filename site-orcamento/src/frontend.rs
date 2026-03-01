slint::include_modules!();
use slint::Model;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct CreateClient {
    nome: String,
    email: String,
    telefone: String,
    endereco: String,
}

#[derive(Serialize, Deserialize)]
struct CreateProduct {
    codigo: String,
    nome: String,
    valor: f64,
    estoque: i32,
    categoria: String,
}

#[derive(Serialize, Deserialize)]
struct CreateQuotationItem {
    produto_id: i32,
    quantidade: i32,
    valor_unitario: f64,
}

#[derive(Serialize, Deserialize)]
struct CreateQuotation {
    cliente_id: i32,
    itens: Vec<CreateQuotationItem>,
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn main_wasm() {
    // This provides better error messages in the browser console
    console_error_panic_hook::set_once();
    spawn_local(run_app());
}

pub async fn run_app() {
    let ui = AppWindow::new().unwrap();
    let ui_handle = ui.as_weak();

    let clients_cache: std::sync::Arc<std::sync::Mutex<Vec<ClientItem>>> = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let products_cache: std::sync::Arc<std::sync::Mutex<Vec<ProductSearchItem>>> = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));

    // Initial data fetch
    fetch_dashboard_data(ui_handle.clone()).await;

    ui.on_navigate({
        let ui_handle = ui_handle.clone();
        let clients_cache = clients_cache.clone();
        let products_cache = products_cache.clone();
        move |page| {
            println!("Navigating to {}", page);
            let ui_handle = ui_handle.clone();
            if page == "dashboard" {
                spawn_local(async move {
                    fetch_dashboard_data(ui_handle).await;
                });
            } else if page == "orcamentos" {
                let clients_cache = clients_cache.clone();
                let products_cache = products_cache.clone();
                spawn_local(async move {
                    fetch_quotation_lists(ui_handle, clients_cache, products_cache).await;
                });
            }
        }
    });

    ui.on_search_clients({
        let ui_handle = ui_handle.clone();
        let clients_cache = clients_cache.clone();
        move |query| {
            if let Some(ui) = ui_handle.upgrade() {
                let query = query.to_string().to_lowercase();
                let cache = clients_cache.lock().unwrap();
                let filtered: Vec<ClientItem> = if query.is_empty() {
                    cache.clone()
                } else {
                    cache.iter().filter(|c| c.nome.to_string().to_lowercase().contains(&query)).cloned().collect()
                };
                let model = std::rc::Rc::new(slint::VecModel::from(filtered));
                ui.set_client_list(model.into());
            }
        }
    });

    ui.on_search_products({
        let ui_handle = ui_handle.clone();
        let products_cache = products_cache.clone();
        move |query| {
            if let Some(ui) = ui_handle.upgrade() {
                let query = query.to_string().to_lowercase();
                let cache = products_cache.lock().unwrap();
                let filtered: Vec<ProductSearchItem> = if query.is_empty() {
                    cache.clone()
                } else {
                    cache.iter().filter(|p| {
                        let n = p.nome.to_string().to_lowercase();
                        let c = p.codigo.to_string().to_lowercase();
                        n.contains(&query) || c.contains(&query)
                    }).cloned().collect()
                };
                let model = std::rc::Rc::new(slint::VecModel::from(filtered));
                ui.set_product_list(model.into());
            }
        }
    });

    // Quotation logic
    ui.on_add_item({
        let ui_handle = ui_handle.clone();
        move |id, name, price| {
            if let Some(ui) = ui_handle.upgrade() {
                let mut items: Vec<ProductItem> = ui.get_current_items().iter().collect();
                // Check if already exists
                let mut found = false;
                for item in items.iter_mut() {
                    if item.id == id {
                        item.quantity += 1;
                        found = true;
                        break;
                    }
                }
                if !found {
                    items.push(ProductItem {
                        id,
                        name: name.into(),
                        quantity: 1,
                        price,
                    });
                }
                
                let total: f32 = items.iter().map(|i| i.price * i.quantity as f32).sum();
                
                let model = std::rc::Rc::new(slint::VecModel::from(items));
                ui.set_current_items(model.into());
                ui.set_total_quotation(total);
            }
        }
    });

    ui.on_remove_item({
        let ui_handle = ui_handle.clone();
        move |index| {
            if let Some(ui) = ui_handle.upgrade() {
                let mut items: Vec<ProductItem> = ui.get_current_items().iter().collect();
                if (index as usize) < items.len() {
                    items.remove(index as usize);
                }
                let total: f32 = items.iter().map(|i| i.price * i.quantity as f32).sum();
                let model = std::rc::Rc::new(slint::VecModel::from(items));
                ui.set_current_items(model.into());
                ui.set_total_quotation(total);
            }
        }
    });

    ui.on_save_quotation({
        let ui_handle = ui_handle.clone();
        move |client_id, items_rc: slint::ModelRc<ProductItem>| {
            let mut items = Vec::new();
            for item in items_rc.iter() {
                items.push(CreateQuotationItem {
                    produto_id: item.id,
                    quantidade: item.quantity,
                    valor_unitario: item.price as f64,
                });
            }
            
            let quotation = CreateQuotation {
                cliente_id: client_id,
                itens: items,
            };
            
            let ui_handle = ui_handle.clone();
            spawn_local(async move {
                let http_client = reqwest::Client::new();
                match http_client.post("http://localhost:3000/api/quotations")
                    .json(&quotation)
                    .send()
                    .await {
                    Ok(_) => {
                        println!("Quotation saved successfully");
                        let ui_handle_inner = ui_handle.clone();
                        let _ = slint::invoke_from_event_loop(move || {
                            if let Some(ui) = ui_handle_inner.upgrade() {
                                ui.set_current_items(std::rc::Rc::new(slint::VecModel::default()).into());
                                ui.set_total_quotation(0.0);
                                ui.set_active_page("dashboard".into());
                            }
                        });
                    },
                    Err(e) => eprintln!("Error saving quotation: {:?}", e),
                }
            });
        }
    });

    ui.on_refresh_lists({
        let ui_handle = ui_handle.clone();
        let clients_cache = clients_cache.clone();
        let products_cache = products_cache.clone();
        move || {
            let ui_handle = ui_handle.clone();
            let clients_cache = clients_cache.clone();
            let products_cache = products_cache.clone();
            spawn_local(async move {
                fetch_quotation_lists(ui_handle, clients_cache, products_cache).await;
            });
        }
    });

    ui.on_create_client({
        let _ui_handle = ui_handle.clone();
        move |name, email, phone, address| {
            let client = CreateClient {
                nome: name.to_string(),
                email: email.to_string(),
                telefone: phone.to_string(),
                endereco: address.to_string(),
            };
            
            spawn_local(async move {
                let http_client = reqwest::Client::new();
                match http_client.post("http://localhost:3000/api/clients")
                    .json(&client)
                    .send()
                    .await {
                    Ok(_) => println!("Client created successfully"),
                    Err(e) => eprintln!("Error creating client: {:?}", e),
                }
            });
        }
    });

    ui.on_create_product({
        let _ui_handle = ui_handle.clone();
        move |code, name, price_str, stock_str, category| {
            // Replace comma with dot to support Brazilian format (e.g., "150,00" -> "150.00")
            let price_clean = price_str.to_string().replace(",", ".");
            let parsed_price = price_clean.parse::<f64>().unwrap_or(0.0);
            
            // Clean up stock string just in case
            let stock_clean = stock_str.to_string().trim().to_string();
            let parsed_stock = stock_clean.parse::<i32>().unwrap_or(0);

            let product = CreateProduct {
                codigo: code.to_string(),
                nome: name.to_string(),
                valor: parsed_price,
                estoque: parsed_stock,
                categoria: category.to_string(),
            };
            
            spawn_local(async move {
                let http_client = reqwest::Client::new();
                match http_client.post("http://localhost:3000/api/products")
                    .json(&product)
                    .send()
                    .await {
                    Ok(_) => println!("Product created successfully"),
                    Err(e) => eprintln!("Error creating product: {:?}", e),
                }
            });
        }
    });

    ui.run().unwrap();
}

async fn fetch_dashboard_data(ui_handle: slint::Weak<AppWindow>) {
    // In a real WASM app, we'd use gloo-net or reqwest
    // For this demonstration, we'll simulate the fetch or use a simple reqwest call if mocked
    let (val, count) = if let Ok(resp) = reqwest::get("http://localhost:3000/api/dashboard").await {
        if let Ok(data) = resp.json::<(f64, i32)>().await {
            data
        } else {
            (0.0, 0)
        }
    } else {
        (0.0, 0)
    };

    let _ = slint::invoke_from_event_loop(move || {
        if let Some(ui) = ui_handle.upgrade() {
            ui.set_total_value(format!("R$ {:.2}", val).into());
            ui.set_quotation_count(count.to_string().into());
        }
    });
}

#[cfg(target_arch = "wasm32")]
fn spawn_local<F: std::future::Future<Output = ()> + 'static>(f: F) {
    wasm_bindgen_futures::spawn_local(f);
}

#[derive(Serialize, Deserialize, Clone)]
struct FetchClient {
    id: i32,
    nome: String,
    email: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct FetchProduct {
    id: i32,
    codigo: String,
    nome: String,
    valor: f64,
}

#[cfg(not(target_arch = "wasm32"))]
fn spawn_local<F: std::future::Future<Output = ()> + Send + 'static>(f: F) {
    tokio::spawn(f);
}

async fn fetch_quotation_lists(
    ui_handle: slint::Weak<AppWindow>,
    clients_cache: std::sync::Arc<std::sync::Mutex<Vec<ClientItem>>>,
    products_cache: std::sync::Arc<std::sync::Mutex<Vec<ProductSearchItem>>>
) {
    let http_client = reqwest::Client::new();
    
    let clients_res = http_client.get("http://localhost:3000/api/clients").send().await;
    let products_res = http_client.get("http://localhost:3000/api/products").send().await;

    let clients_data: Vec<FetchClient> = if let Ok(resp) = clients_res {
        resp.json().await.unwrap_or_default()
    } else {
        Vec::new()
    };

    let products_data: Vec<FetchProduct> = if let Ok(resp) = products_res {
        resp.json().await.unwrap_or_default()
    } else {
        Vec::new()
    };

    let _ = slint::invoke_from_event_loop(move || {
        if let Some(ui) = ui_handle.upgrade() {
            let slint_clients: Vec<ClientItem> = clients_data.into_iter().map(|c| ClientItem {
                id: c.id,
                nome: c.nome.into(),
                email: c.email.into(),
            }).collect();
            
            if let Ok(mut cache) = clients_cache.lock() {
                *cache = slint_clients.clone();
            }
            
            let clients_model = std::rc::Rc::new(slint::VecModel::from(slint_clients));
            ui.set_client_list(clients_model.into());
            
            let slint_products: Vec<ProductSearchItem> = products_data.into_iter().map(|p| ProductSearchItem {
                id: p.id,
                codigo: p.codigo.into(),
                nome: p.nome.into(),
                valor: p.valor as f32,
            }).collect();
            
            if let Ok(mut cache) = products_cache.lock() {
                *cache = slint_products.clone();
            }
            
            let products_model = std::rc::Rc::new(slint::VecModel::from(slint_products));
            ui.set_product_list(products_model.into());
        }
    });
}
