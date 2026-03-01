pub const API_BASE_URL: &str = "http://localhost:3000";

pub fn get_url(path: &str) -> String {
    format!("{}{}", API_BASE_URL, path)
}

pub const ENDPOINT_DASHBOARD: &str = "/api/dashboard";
pub const ENDPOINT_CLIENTS: &str = "/api/clients";
pub const ENDPOINT_PRODUCTS: &str = "/api/products";
pub const ENDPOINT_QUOTATIONS: &str = "/api/quotations";
