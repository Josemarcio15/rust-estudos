-- Tabelas para o sistema de orçamentos (MariaDB)

CREATE TABLE IF NOT EXISTS clientes (
    id INT AUTO_INCREMENT PRIMARY KEY,
    nome VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    telefone VARCHAR(20),
    endereco TEXT
);

CREATE TABLE IF NOT EXISTS produtos (
    id INT AUTO_INCREMENT PRIMARY KEY,
    codigo VARCHAR(50) UNIQUE NOT NULL,
    nome VARCHAR(255) NOT NULL,
    valor DOUBLE NOT NULL,
    estoque INT NOT NULL DEFAULT 0,
    categoria VARCHAR(100)
);

CREATE TABLE IF NOT EXISTS orcamentos (
    id INT AUTO_INCREMENT PRIMARY KEY,
    cliente_id INT NOT NULL,
    data_criacao DATETIME NOT NULL,
    valor_total DOUBLE NOT NULL,
    FOREIGN KEY (cliente_id) REFERENCES clientes(id)
);

CREATE TABLE IF NOT EXISTS itens_orcamento (
    id INT AUTO_INCREMENT PRIMARY KEY,
    orcamento_id INT NOT NULL,
    produto_id INT NOT NULL,
    quantidade INT NOT NULL,
    valor_unitario DOUBLE NOT NULL,
    FOREIGN KEY (orcamento_id) REFERENCES orcamentos(id),
    FOREIGN KEY (produto_id) REFERENCES produtos(id)
    
);
