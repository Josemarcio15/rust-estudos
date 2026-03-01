mod db;
pub mod entities;
mod api;
mod frontend;
mod api_config;

#[tokio::main]
async fn main() {
    std::env::set_var("SLINT_COLOR_SCHEME", "light");
    tracing_subscriber::fmt::init();
    
    // Start API in a separate task
    let _ = tokio::spawn(async {
        let app = api::app();
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        axum::serve(listener, app).await.unwrap();
    });

    println!("API started on http://localhost:3000");
    println!("Starting Slint UI...");
    
    // Run the app (this will block until the window is closed)
    frontend::run_app().await;
}
