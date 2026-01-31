// slint::include_modules!();
// mod db;

// fn main() {
//     dotenvy::dotenv().ok();
//     let app = AppWindow::new().unwrap();

//     let app_weak = app.as_weak();

//     app.on_dashboard_load(move || {
//         let app = app_weak.unwrap();


//         app.set_total_emDias(mensalidade.to_string().into());
        
//         app.set_total_debitos("42".into());
//         app.set_total_diarias("42".into());

//         let total = db::queries::total_clientes();
//         app.set_total_alunos(total.to_string().into());
//         println!("Entrou");
//     });

//     app.run().unwrap();
// }

mod db;
mod queries;
mod services;
mod schema;

slint::include_modules!();

fn main() {
    let pool = db::criar_pool_banco();

    let resumo = services::dashboard::carregar_dashboard(&pool)
        .expect("Erro ao carregar dashboard");

    let app = AppWindow::new().expect("Erro ao criar AppWindow");

app.set_total_clientes(resumo.total_clientes.to_string().into());
app.set_clientes_em_dia(resumo.clientes_em_dia.to_string().into());
app.set_clientes_atrasados(resumo.clientes_atrasados.to_string().into());
app.set_plano_diario(resumo.plano_diario.to_string().into());
// app.set_plano_mensal(resumo.plano_mensal.to_string().into());
// app.set_plano_trimestral(resumo.plano_trimestral.to_string().into());
// app.set_plano_semestral(resumo.plano_semestral.to_string().into());
// app.set_plano_anual(resumo.plano_anual.to_string().into());

    app.run().expect("Erro ao executar UI");
}
