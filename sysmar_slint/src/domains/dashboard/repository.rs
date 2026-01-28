mod db;

fn captura() -> i64{
    let mensalidade = db::queries::total_mensal();

}