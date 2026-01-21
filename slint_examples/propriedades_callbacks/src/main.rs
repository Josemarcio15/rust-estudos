slint::include_modules!();

fn main() {
    let app = AppMain::new().unwrap();
    let weak_app = app.as_weak();

    app.on_chamando(move || {
        if let Some(app) = weak_app.upgrade(){
            let atual = app.get_contador();
            app.set_contador(atual + 1);
        }
    });
    app.run().unwrap();

}
