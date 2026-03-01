-- Create Clients table
CREATE TABLE IF NOT EXISTS clients (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    phone VARCHAR(20),
    address TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create Products table
CREATE TABLE IF NOT EXISTS products (
    id INT AUTO_INCREMENT PRIMARY KEY,
    code VARCHAR(50) UNIQUE NOT NULL,
    name VARCHAR(255) NOT NULL,
    value DECIMAL(10, 2) NOT NULL,
    stock INT DEFAULT 0,
    category VARCHAR(100)
);

-- Create Quotations table
CREATE TABLE IF NOT EXISTS quotations (
    id INT AUTO_INCREMENT PRIMARY KEY,
    client_id INT NOT NULL,
    date DATE NOT NULL,
    total_value DECIMAL(10, 2) NOT NULL,
    status ENUM('draft', 'sent', 'accepted', 'rejected') DEFAULT 'draft',
    FOREIGN KEY (client_id) REFERENCES clients(id)
);

-- Create QuotationItems table
CREATE TABLE IF NOT EXISTS quotation_items (
    id INT AUTO_INCREMENT PRIMARY KEY,
    quotation_id INT NOT NULL,
    product_id INT NOT NULL,
    quantity INT NOT NULL,
    unit_price DECIMAL(10, 2) NOT NULL,
    FOREIGN KEY (quotation_id) REFERENCES quotations(id),
    FOREIGN KEY (product_id) REFERENCES products(id)
);
