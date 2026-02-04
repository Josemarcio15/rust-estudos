slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let login = LoginWindow::new()?;
    let principal = PrincipalWindow::new()?;

    principal.hide().unwrap();

    let principal_handle = principal.as_weak();

    login.on_entrar(move || {
        if let Some(p) = principal_handle.upgrade() {
            p.show().unwrap();
        }
    });

    login.run()
}
