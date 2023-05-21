CREATE DATABASE IF NOT EXISTS iletter_db;
USE iletter_db;

CREATE TABLE IF NOT EXISTS iletter_db.manager (
    id INT AUTO_INCREMENT,
    first_name VARCHAR(60) NOT NULL,
    middle_name VARCHAR(60),
    first_last_name VARCHAR(60) NOT NULL,
    second_last_name VARCHAR(60),
    identification VARCHAR(120) NOT NULL,
    date_of_birth DATE NOT NULL,
    email_address VARCHAR(254) NOT NULL,
    phone_number VARCHAR(20) NOT NULL,
    residence_address VARCHAR(300) NOT NULL,
    date_of_hire DATE NOT NULL,
    basic_salary DECIMAL(10, 5) NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS iletter_db.inventory (
    id INT AUTO_INCREMENT,
    description TEXT,
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS iletter_db.book (
    isbn UUID,
    name VARCHAR(255) NOT NULL,
    author_name VARCHAR(255) NOT NULL,
    publisher_name VARCHAR(255) NOT NULL,
    number_of_pages SMALLINT NOT NULL,
    publication_date DATE NOT NULL,
    PRIMARY KEY (isbn)
);

CREATE TABLE IF NOT EXISTS iletter_db.book_lot (
    id INT AUTO_INCREMENT,
    book_isbn UUID NOT NULL,
    price_per_unit DECIMAL(10, 5) NOT NULL,
    available_units SMALLINT NOT NULL,
    inventory_id INT NOT NULL,
    PRIMARY KEY (id),
    CONSTRAINT book_lot_book FOREIGN KEY (book_isbn)
    REFERENCES iletter_db.book (isbn) ON UPDATE CASCADE ON DELETE CASCADE,
    CONSTRAINT book_lot_inventory FOREIGN KEY (inventory_id)
    REFERENCES iletter_db.inventory (id) ON UPDATE CASCADE ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS iletter_db.branch_office (
    id INT AUTO_INCREMENT,
    inventory_id INT,
    manager_id INT,
    PRIMARY KEY (id),
    CONSTRAINT branch_office_inventory FOREIGN KEY (inventory_id)
    REFERENCES iletter_db.inventory (id) ON UPDATE CASCADE ON DELETE SET NULL,
    CONSTRAINT branch_office_manager FOREIGN KEY (manager_id)
    REFERENCES iletter_db.manager (id) ON UPDATE CASCADE ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS iletter_db.worker (
    id INT AUTO_INCREMENT,
    first_name VARCHAR(60) NOT NULL,
    middle_name VARCHAR(60),
    first_last_name VARCHAR(60) NOT NULL,
    second_last_name VARCHAR(60),
    identification VARCHAR(120) NOT NULL,
    date_of_birth DATE NOT NULL,
    email_address VARCHAR(254) NOT NULL,
    phone_number VARCHAR(20) NOT NULL,
    residence_address VARCHAR(300) NOT NULL,
    date_of_hire DATE NOT NULL,
    role VARCHAR(126) NOT NULL,
    basic_salary DECIMAL(10, 5) NOT NULL,
    branch_office_id INT NOT NULL,
    PRIMARY KEY (id),
    CONSTRAINT worker_branch_office FOREIGN KEY (branch_office_id)
    REFERENCES iletter_db.branch_office (id) ON UPDATE CASCADE ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS iletter_db.client (
    id INT AUTO_INCREMENT,
    first_name VARCHAR(60) NOT NULL,
    middle_name VARCHAR(60),
    first_last_name VARCHAR(60) NOT NULL,
    second_last_name VARCHAR(60),
    identification VARCHAR(120) NOT NULL,
    email_address VARCHAR(254) NOT NULL,
    phone_number VARCHAR(20) NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS iletter_db.branch_office_clients (
    client_id INT NOT NULL,
    branch_office_id INT NOT NULL,
    PRIMARY KEY (client_id, branch_office_id),
    CONSTRAINT branch_office_clients_clients FOREIGN KEY (client_id)
    REFERENCES iletter_db.client (id) ON UPDATE CASCADE ON DELETE CASCADE,
    CONSTRAINT branch_office_clients_branch_offices
    FOREIGN KEY (branch_office_id)
    REFERENCES iletter_db.branch_office (id) ON UPDATE CASCADE ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS iletter_db.order_details (
    id INT AUTO_INCREMENT,
    book_name VARCHAR(255) NOT NULL,
    quantity SMALLINT NOT NULL,
    total_price DECIMAL(10, 5) NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS iletter_db.order (
    id INT AUTO_INCREMENT,
    branch_office_id INT NOT NULL,
    order_details_id INT NOT NULL,
    client_id INT NOT NULL,
    PRIMARY KEY (id),
    CONSTRAINT order_branch_office FOREIGN KEY (branch_office_id)
    REFERENCES iletter_db.branch_office (id)
    ON UPDATE CASCADE ON DELETE CASCADE,
    CONSTRAINT order_order_details FOREIGN KEY (order_details_id)
    REFERENCES iletter_db.order_details (id)
    ON UPDATE CASCADE ON DELETE CASCADE,
    CONSTRAINT order_client FOREIGN KEY (client_id)
    REFERENCES iletter_db.client (id) ON UPDATE CASCADE ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS iletter_db.administrator_details (
    id INT AUTO_INCREMENT,
    first_name VARCHAR(60) NOT NULL,
    middle_name VARCHAR(60),
    first_last_name VARCHAR(60) NOT NULL,
    second_last_name VARCHAR(60),
    identification VARCHAR(120) NOT NULL,
    email_address VARCHAR(254) NOT NULL,
    phone_number VARCHAR(20) NOT NULL,
    branch_office_id INT,
    PRIMARY KEY (id),
    CONSTRAINT administrator_details_branch_office
    FOREIGN KEY (branch_office_id) REFERENCES iletter_db.branch_office (id)
    ON UPDATE CASCADE ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS iletter_db.administrator (
    id INT AUTO_INCREMENT,
    user_name VARCHAR(50) NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    administrator_details_id INT NOT NULL,
    PRIMARY KEY (id),
    CONSTRAINT administrator_administrator_details
    FOREIGN KEY (administrator_details_id)
    REFERENCES iletter_db.administrator_details (id)
    ON UPDATE CASCADE ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS iletter_db.process_audit (
    id INT AUTO_INCREMENT,
    manipulation_type ENUM("CREATE", "UPDATE", "DELETE") NOT NULL,
    date_time DATETIME NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS iletter_db.process_audit_administrators (
    process_audit_id INT NOT NULL,
    administrator_id INT NOT NULL,
    PRIMARY KEY (process_audit_id, administrator_id),
    CONSTRAINT process_audit_administrators_process_audits
    FOREIGN KEY (process_audit_id) REFERENCES iletter_db.process_audit (id)
    ON UPDATE CASCADE ON DELETE CASCADE,
    CONSTRAINT process_audit_administrators_administrators
    FOREIGN KEY (administrator_id) REFERENCES iletter_db.administrator (id)
    ON UPDATE CASCADE ON DELETE CASCADE
);
