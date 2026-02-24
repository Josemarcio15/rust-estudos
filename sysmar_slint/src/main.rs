//use crate::queries::dashboard::generic;

mod db;
mod queries;
mod services;
mod entities;

slint::include_modules!();

#[tokio::main]
async fn main() -> Result<(), slint::PlatformError> {
    // 1. Inicializa o banco (Certifique-se que db::criar_pool_banco retorna a conexão direta ou use .expect)
    let pool = db::criar_pool_banco().await;

    // 2. Carrega dados iniciais
    let resumo = services::dashboard::carregar_dashboard_ui(&pool)
        .await
        .expect("Erro ao carregar dashboard");

    println!("Resumo dashboard carregado com sucesso.");

    // 3. Cria a Janela
    let app = AppWindow::new()?;

    // 4. Preenche os dados iniciais na tela
    app.set_total_clientes(resumo.total_clientes);
    app.set_clientes_em_dia(resumo.clientes_em_dia);
    app.set_clientes_atrasados(resumo.clientes_atrasados);
    app.set_plano_diario(resumo.plano_diario);
    app.set_plano_mensal(resumo.plano_mensal);
    app.set_plano_trimestral(resumo.plano_trimestral);
    app.set_plano_semestral(resumo.plano_semestral);
    app.set_plano_anual(resumo.plano_anual);

    app.set_percentual_clientes_em_dia(resumo.percentual_clientes_em_dia);
    app.set_percentual_clientes_atrasados(resumo.percentual_clientes_atrasados);
    app.set_percentual_plano_diario(resumo.percentual_plano_diario);
    app.set_percentual_plano_mensal(resumo.percentual_plano_mensal);
    app.set_percentual_plano_trimestral(resumo.percentual_plano_trimestral);
    app.set_percentual_plano_semestral(resumo.percentual_plano_semestral);
    app.set_percentual_plano_anual(resumo.percentual_plano_anual);

    // --- PREPARAÇÃO PARA O CALLBACK ---
    let app_weak = app.as_weak();
    let pool_callback = pool.clone();

    // 5. Registro da ação do botão
    app.on_filtro_dashboard(move |i_dia, i_mes, i_ano, f_dia, f_mes, f_ano| {
        // Clonamos para dentro do closure do Slint
        let pool_for_task = pool_callback.clone();
        let ui_handle = app_weak.clone();

        // Debug: Ver os valores recebidos da UI
        println!("Filtro acionado: De {}/{}/{} até {}/{}/{}", 
            i_dia, i_mes, i_ano, f_dia, f_mes, f_ano
        );

        // Formatamos as datas (garantindo o padrão AAAA-MM-DD com zeros à esquerda)
        let data_ini = format!("{:0>4}-{:0>2}-{:0>2}", i_ano, i_mes, i_dia);
        let data_fim = format!("{:0>4}-{:0>2}-{:0>2}", f_ano, f_mes, f_dia);

        // Criamos a tarefa assíncrona para não travar a interface
        tokio::spawn(async move {
            let resultado = services::dashboard::generic::calcular_valores(
                &pool_for_task, 
                &data_ini, 
                &data_fim
            ).await;

match resultado {
    Ok(dados) => {
ui_handle.upgrade_in_event_loop(move |ui| {
    // Dividimos por 100.0 para transformar centavos em Reais
    ui.set_valor_diario(dados.total_diario as f32 / 100.0);
    ui.set_valor_mensal(dados.total_mensal as f32 / 100.0);
    ui.set_valor_trimestral(dados.total_trimestral as f32 / 100.0);
    ui.set_valor_semestral(dados.total_semestral as f32 / 100.0);
    ui.set_valor_anual(dados.total_anual as f32 / 100.0);
    ui.set_valor_total_geral(dados.total_geral as f32 / 100.0);
}).expect("Erro ao atualizar UI");
    }
    Err(e) => {
        eprintln!("Erro na query: {:?}", e);
    }
}
        });
    });

    app.run()
}