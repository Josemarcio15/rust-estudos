slint::include_modules!();
mod db;

fn main() {


    let app = AppWindow::new().unwrap();
    app.set_total_emDias("42".into());
    app.set_total_debitos("42".into());
    app.set_total_diarias("42".into());
        
        
        
    app.set_total_alunos(db::queries::total_clientes().to_string().into());

    
    app.run().unwrap();

}
