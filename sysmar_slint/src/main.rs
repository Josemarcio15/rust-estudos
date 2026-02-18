mod db;
mod queries;
mod services;
mod entities;


// Importa a shared string para lidar com textos do Slint
// use slint::SharedString;

slint::include_modules!();

#[tokio::main]
async fn main() -> Result<(), slint::PlatformError> {
    // 1. Inicializa o banco
    let pool = db::criar_pool_banco().await;

    // 2. Carrega dados iniciais (Assíncrono)
    let resumo = services::dashboard::carregar_dashboard_ui(&pool)
        .await
        .expect("Erro ao carregar dashboard");

    // Debug no terminal
    println!("Resumo dashboard carregado com sucesso.");
    // ... (Seus prints de debug continuam aqui se quiser) ...

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

    // --- PREPARAÇÃO PARA O CALLBACK (Botão Filtrar) ---

    // A. Clonamos o handle da janela (Weak) para usar DENTRO do callback (se precisar atualizar a tela depois)
    let app_weak = app.as_weak();

    // B. Clonamos o pool do banco para mover para DENTRO do callback
    let pool_callback = pool.clone();

    // 5. Registramos a ação do botão. ATENÇÃO: Chamamos no 'app', não no 'ui_weak'
    app.on_filtro_dashboard(move |i_dia, i_mes, i_ano, f_dia, f_mes, f_ano| {
        
        // Debug: Ver se chegou tudo certo
        println!("Filtro acionado: De {}/{}/{} até {}/{}/{}", 
            i_dia, i_mes, i_ano, f_dia, f_mes, f_ano
        );

        // Conversão: SharedString -> i32
        let _inicio_dia = i_dia.parse::<i32>().unwrap_or(1);
        let _inicio_mes = i_mes.parse::<i32>().unwrap_or(1);
        let _inicio_ano = i_ano.parse::<i32>().unwrap_or(2026);

        let _fim_dia = f_dia.parse::<i32>().unwrap_or(31);
        let _fim_mes = f_mes.parse::<i32>().unwrap_or(12);
        let _fim_ano = f_ano.parse::<i32>().unwrap_or(2026);

        // --- LÓGICA DO FILTRO AQUI ---
        // Aqui você usa 'pool_callback' para chamar sua query
        
        // Exemplo de como seria a chamada (descomente quando criar a função):
        /*
        let resultado_filtro = services::financeiro::filtrar_por_periodo(
            &pool_callback, 
            inicio_dia, inicio_mes, inicio_ano, 
            fim_dia, fim_mes, fim_ano
        );
        */

        // --- DEVOLVER PARA A TELA ---
        // Se você precisar mostrar o valor na tela, use o app_weak assim:
        /*
        let ui = app_weak.unwrap(); // Recupera a conexão com a tela
        ui.set_total_filtrado( resultado_filtro.valor ); 
        */
    });

    // 6. Roda a aplicação
    app.run()
}