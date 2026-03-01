// Módulos do backend — não são suportados no WASM (dependem de tokio/mio/etc.)
#[cfg(not(target_arch = "wasm32"))]
pub mod db;
#[cfg(not(target_arch = "wasm32"))]
pub mod entities;
#[cfg(not(target_arch = "wasm32"))]
pub mod api;

// Módulos compartilhados
pub mod frontend;
pub mod api_config;
