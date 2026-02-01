mod db;
mod queries;
mod services;
mod schema;

slint::include_modules!();

fn main() {
    let pool = db::criar_pool_banco();

    let resumo = services::dashboard::carregar_dashboard_ui(&pool)
        .expect("Erro ao carregar dashboard");
    

    let app = AppWindow::new().expect("Erro ao criar AppWindow");

app.set_total_clientes(resumo.total_clientes);
app.set_clientes_em_dia(resumo.clientes_em_dia);
app.set_clientes_atrasados(resumo.clientes_atrasados);
app.set_plano_diario(resumo.plano_diario);
app.set_plano_mensal(resumo.plano_mensal);
app.set_plano_trimestral(resumo.plano_trimestral);
app.set_plano_semestral(resumo.plano_semestral);
app.set_plano_anual(resumo.plano_anual);

// app.set_percentual_clientes_em_dia(resumo.percentual_clientes_em_dia);
// app.set_percentual_clientes_atrasados(resumo.percentual_clientes_atrasados);
// app.set_percentual_plano_diario(resumo.percentual_plano_diario);
// app.set_percentual_plano_mensal(resumo.percentual_plano_mensal);
// app.set_percentual_plano_trimestral(resumo.percentual_plano_trimestral);
// app.set_percentual_plano_semestral(resumo.percentual_plano_semestral);
// app.set_percentual_plano_anual(resumo.percentual_plano_anual);

    app.run().expect("Erro ao executar UI");
}
