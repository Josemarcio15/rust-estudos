
# dependencias 
sudo apt install libmariadb-dev libmariadb-dev-compat pkg-config

## estrutura do projeto inicial
├── build.rs
├── Cargo.lock
├── Cargo.toml
├── diesel.toml
├── migrations
│   └── 2026-01-28-205445-0000_create_clientes
│       ├── down.sql
│       └── up.sql
├── README.MD
└── src
    ├── assets
    │   ├── image1.jpg
    │   ├── image.jpeg
    │   └── image.jpg
    ├── db
    │   ├── connection.rs
    │   └── mod.rs
    ├── main.rs
    ├── models
    │   ├── dashboard
    │   │   ├── mod.rs
    │   │   └── relatorio.rs
    │   └── financeiro
    │       ├── mod.rs
    │       └── pagamentos.rs
    ├── queries
    │   ├── dashboard
    │   │   ├── generic.rs
    │   │   └── mod.rs
    │   ├── financeiro
    │   │   ├── generic.rs
    │   │   └── mod.rs
    │   └── mod.rs
    ├── schema.rs
    ├── services
    │   ├── dashboard
    │   │   └── mod.rs
    │   ├── financeiro
    │   └── mod.rs
    └── ui
        ├── app.slint
        └── pages
            ├── cadastro.slint
            ├── dashboard.slint
            └── style
                └── shadow.slint