slint::include_modules!();
mod db;

fn main() {
    dotenvy::dotenv().ok();
    let app = AppWindow::new().unwrap();

    let app_weak = app.as_weak();

    app.on_dashboard_load(move || {
        let app = app_weak.unwrap();


        app.set_total_emDias(mensalidade.to_string().into());
        
        app.set_total_debitos("42".into());
        app.set_total_diarias("42".into());

        let total = db::queries::total_clientes();
        app.set_total_alunos(total.to_string().into());
        println!("Entrou");
    });

    app.run().unwrap();
}
