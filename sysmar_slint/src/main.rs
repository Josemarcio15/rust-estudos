mod db;
mod queries;
mod services;
mod schema;
mod models;

// Importa a shared string para lidar com textos do Slint

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    // 1. Inicializa o banco
    let pool = db::criar_pool_banco();

    // 2.1 Carrega dados financeiros iniciais (Mês atual)
    use chrono::{Local, Datelike, NaiveDate};
    let hoje = Local::now().naive_local().date();
    let start_date_init = NaiveDate::from_ymd_opt(hoje.year(), hoje.month(), 1).unwrap();
    let end_date_init = hoje;

    // 2. Carrega dados iniciais (Sincrono)
    let resumo = services::dashboard::carregar_dashboard_ui(&pool, start_date_init, end_date_init)
        .expect("Erro ao carregar dashboard");

    let resumo_financeiro = services::financeiro::get_resumo_financeiro(&pool, start_date_init, end_date_init);

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

    // 4.1 Preenche os dados financeiros iniciais
    app.set_total_financeiro_diario(format!("{:.2}", resumo_financeiro.diario).into());
    app.set_total_financeiro_mensal(format!("{:.2}", resumo_financeiro.mensal).into());
    app.set_total_financeiro_trimestral(format!("{:.2}", resumo_financeiro.trimestral).into());
    app.set_total_financeiro_semestral(format!("{:.2}", resumo_financeiro.semestral).into());
    app.set_total_financeiro_anual(format!("{:.2}", resumo_financeiro.anual).into());
    app.set_total_financeiro_geral(format!("{:.2}", resumo_financeiro.total).into());

    // --- PREPARAÇÃO PARA O CALLBACK (Botão Filtrar) ---

    // A. Clonamos o handle da janela (Weak) para usar DENTRO do callback (se precisar atualizar a tela depois)
    let app_weak = app.as_weak();

    // B. Clonamos o pool do banco para mover para DENTRO do callback
    let pool_callback = pool.clone();

    // 5. Registramos a ação do botão. ATENÇÃO: Chamamos no 'app', não no 'ui_weak'
    app.on_filtro_dashboard(move |i_dia, i_mes, i_ano, f_dia, f_mes, f_ano| {
        use chrono::NaiveDate;
        
        // Debug: Ver se chegou tudo certo
        println!("Filtro acionado: De {}/{}/{} até {}/{}/{}", 
            i_dia, i_mes, i_ano, f_dia, f_mes, f_ano
        );

        // Conversão: SharedString -> i32 -> NaiveDate
        let inicio_dia = i_dia.parse::<u32>().unwrap_or(1);
        let inicio_mes = i_mes.parse::<u32>().unwrap_or(1);
        let inicio_ano = i_ano.parse::<i32>().unwrap_or(2026);

        let fim_dia = f_dia.parse::<u32>().unwrap_or(31);
        let fim_mes = f_mes.parse::<u32>().unwrap_or(12);
        let fim_ano = f_ano.parse::<i32>().unwrap_or(2026);

        let start_date = NaiveDate::from_ymd_opt(inicio_ano, inicio_mes, inicio_dia)
            .unwrap_or(NaiveDate::from_ymd_opt(2026, 1, 1).unwrap());
        
        let end_date = NaiveDate::from_ymd_opt(fim_ano, fim_mes, fim_dia)
            .unwrap_or(NaiveDate::from_ymd_opt(2026, 12, 31).unwrap());

        // --- LÓGICA DO FILTRO AQUI ---
        let resumo_financeiro = services::financeiro::get_resumo_financeiro(
            &pool_callback, 
            start_date, 
            end_date
        );

        let resumo_dashboard = services::dashboard::carregar_dashboard_ui(
            &pool_callback,
            start_date,
            end_date
        ).expect("Erro ao filtrar dashboard");

        // --- DEVOLVER PARA A TELA ---
        if let Some(ui) = app_weak.upgrade() {
            ui.set_total_financeiro_diario(format!("{:.2}", resumo_financeiro.diario).into());
            ui.set_total_financeiro_mensal(format!("{:.2}", resumo_financeiro.mensal).into());
            ui.set_total_financeiro_trimestral(format!("{:.2}", resumo_financeiro.trimestral).into());
            ui.set_total_financeiro_semestral(format!("{:.2}", resumo_financeiro.semestral).into());
            ui.set_total_financeiro_anual(format!("{:.2}", resumo_financeiro.anual).into());
            ui.set_total_financeiro_geral(format!("{:.2}", resumo_financeiro.total).into());

            // --- ATUALIZAR CONTADORES DA ESQUERDA ---
            ui.set_total_clientes(resumo_dashboard.total_clientes);
            ui.set_clientes_em_dia(resumo_dashboard.clientes_em_dia);
            ui.set_clientes_atrasados(resumo_dashboard.clientes_atrasados);
            ui.set_plano_diario(resumo_dashboard.plano_diario);
            ui.set_plano_mensal(resumo_dashboard.plano_mensal);
            ui.set_plano_trimestral(resumo_dashboard.plano_trimestral);
            ui.set_plano_semestral(resumo_dashboard.plano_semestral);
            ui.set_plano_anual(resumo_dashboard.plano_anual);

            ui.set_percentual_clientes_em_dia(resumo_dashboard.percentual_clientes_em_dia);
            ui.set_percentual_clientes_atrasados(resumo_dashboard.percentual_clientes_atrasados);
            ui.set_percentual_plano_diario(resumo_dashboard.percentual_plano_diario);
            ui.set_percentual_plano_mensal(resumo_dashboard.percentual_plano_mensal);
            ui.set_percentual_plano_trimestral(resumo_dashboard.percentual_plano_trimestral);
            ui.set_percentual_plano_semestral(resumo_dashboard.percentual_plano_semestral);
            ui.set_percentual_plano_anual(resumo_dashboard.percentual_plano_anual);
        }
    });

    // 6. Roda a aplicação
    app.run()
}