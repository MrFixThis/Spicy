INSERT INTO iletter_db.manager (
    first_name, middle_name, first_last_name, second_last_name, identification,
    date_of_birth, email_address, phone_number, residence_address, date_of_hire,
    basic_salary
)
VALUES
("John", NULL, "Smith", NULL, "1234567890", "1990-01-01", "john@example.com", "1234567890", "123 Main St", "2022-01-01", 50000.00),
("Jane", NULL, "Doe", NULL, "0987654321", "1992-02-02", "jane@example.com", "9876543210", "456 Elm St", "2022-01-01", 55000.00),
("David", NULL, "Johnson", NULL, "9876543210", "1985-03-15", "david@example.com", "9876543210", "789 Oak St", "2022-02-01", 60000.00),
("Sarah", NULL, "Williams", NULL, "3456789012", "1991-07-10", "sarah@example.com", "1234567890", "567 Pine St", "2022-03-01", 52000.00),
("Michael", NULL, "Brown", NULL, "2345678901", "1988-09-20", "michael@example.com", "9876543210", "890 Maple St", "2022-03-01", 54000.00),
("Emily", NULL, "Jones", NULL, "4567890123", "1993-05-05", "emily@example.com", "1234567890", "678 Oak St", "2022-04-01", 51000.00),
("Daniel", NULL, "Davis", NULL, "7654321098", "1987-12-08", "daniel@example.com", "9876543210", "901 Elm St", "2022-04-01", 53000.00),
("Olivia", NULL, "Miller", NULL, "5678901234", "1994-10-25", "olivia@example.com", "1234567890", "123 Pine St", "2022-05-01", 49000.00),
("William", NULL, "Anderson", NULL, "6543210987", "1986-06-18", "william@example.com", "9876543210", "456 Oak St", "2022-05-01", 58000.00),
("Sophia", NULL, "Taylor", NULL, "7890123456", "1995-08-12", "sophia@example.com", "1234567890", "789 Elm St", "2022-06-01", 55000.00),
("James", NULL, "Thomas", NULL, "5432109876", "1989-04-30", "james@example.com", "9876543210", "234 Pine St", "2022-06-01", 57000.00),
("Isabella", NULL, "Jackson", NULL, "0123456789", "1996-11-22", "isabella@example.com", "1234567890", "567 Oak St", "2022-07-01", 53000.00),
("Mia", NULL, "Clark", NULL, "9012345678", "1997-09-14", "mia@example.com", "9876543210", "890 Pine St", "2022-07-01", 51000.00),
("Alexander", NULL, "Harris", NULL, "3210987654", "1983-03-03", "alexander@example.com", "1234567890", "123 Elm St", "2022-08-01", 54000.00),
("Ava", NULL, "Lewis", NULL, "8901234567", "1998-01-26", "ava@example.com", "9876543210", "456 Pine St", "2022-08-01", 52000.00),
("Henry", NULL, "Lee", NULL, "2109876543", "1982-07-07", "henry@example.com", "1234567890", "789 Elm St", "2022-09-01", 59000.00),
("Charlotte", NULL, "Taylor", NULL, "7890123456", "1999-05-18", "charlotte@example.com", "9876543210", "901 Pine St", "2022-09-01", 56000.00),
("Joseph", NULL, "Moore", NULL, "1098765432", "1981-11-09", "joseph@example.com", "1234567890", "234 Elm St", "2022-10-01", 57000.00),
("Amelia", NULL, "Walker", NULL, "0987654321", "2000-03-02", "amelia@example.com", "9876543210", "567 Elm St", "2022-10-01", 53000.00),
("Daniel", NULL, "Hall", NULL, "9876543210", "1980-09-23", "daniel@example.com", "1234567890", "890 Elm St", "2022-11-01", 60000.00);

INSERT INTO iletter_db.inventory (description)
VALUES
("Description 1"), ("Description 2"), ("Description 3"), ("Description 4"),
("Description 5"), ("Description 6"), ("Description 7"), ("Description 8"),
("Description 9"), ("Description 10"), ("Description 11"), ("Description 12"),
("Description 13"), ("Description 14"), ("Description 15"), ("Description 16"),
("Description 17"), ("Description 18"), ("Description 19"), ("Description 20");

INSERT INTO iletter_db.book (
    isbn, name, author_name, publisher_name, number_of_pages, publication_date
)
VALUES
("e51c0a75-cc5f-4d10-bac6-12e1b1f7699a", "Book 1", "Author 1", "Publisher 1", 200, "2022-01-01"),
("04385f6a-9724-4c71-9e5d-6de2396639d2", "Book 2", "Author 2", "Publisher 2", 300, "2022-02-01"),
("7be6f607-4812-4f32-a141-3e0dd88850f7", "Book 3", "Author 3", "Publisher 3", 250, "2022-03-01"),
("2a8270ce-961d-46b1-9f2e-bd6de4f60f2a", "Book 4", "Author 4", "Publisher 4", 180, "2022-04-01"),
("c9964fc6-9cb4-4d5e-b4b2-7bc7dc9a9426", "Book 5", "Author 5", "Publisher 5", 220, "2022-05-01"),
("fb0c2b61-7a8f-4a68-809f-6467802b0646", "Book 6", "Author 6", "Publisher 6", 150, "2022-06-01"),
("a392ff1d-3f6c-41e4-8e11-f496d9777704", "Book 7", "Author 7", "Publisher 7", 280, "2022-07-01"),
("75992561-47a3-4a37-87d1-d8eb5d7a716a", "Book 8", "Author 8", "Publisher 8", 190, "2022-08-01"),
("9a181dca-8459-4fa5-a7ed-836a7d747536", "Book 9", "Author 9", "Publisher 9", 230, "2022-09-01"),
("e6e3999a-833b-43b5-936d-879790af0a7d", "Book 10", "Author 10", "Publisher 10", 210, "2022-10-01"),
("6820d5f7-615b-4a22-bdce-6517f2f822f2", "Book 11", "Author 11", "Publisher 11", 270, "2022-11-01"),
("b5bdcf09-8c45-4c99-a2d9-0f692c74b396", "Book 12", "Author 12", "Publisher 12", 240, "2022-12-01"),
("15b9eef3-2494-4a3e-94b4-493be2a08ce4", "Book 13", "Author 13", "Publisher 13", 170, "2023-01-01"),
("25d10384-8d4c-4d4e-a4b1-6e0ef06b6280", "Book 14", "Author 14", "Publisher 14", 260, "2023-02-01"),
("c550b737-64d1-4fd2-af0f-57a77ef27f33", "Book 15", "Author 15", "Publisher 15", 320, "2023-03-01"),
("c6e8fb3b-8765-4e6f-aa42-ef8a89e0a474", "Book 16", "Author 16", "Publisher 16", 200, "2023-04-01"),
("d31294e5-45dd-4be3-a26a-9b8c90f42c32", "Book 17", "Author 17", "Publisher 17", 280, "2023-05-01"),
("85f06d70-12bc-4d68-a8c5-fb392482f80e", "Book 18", "Author 18", "Publisher 18", 240, "2023-06-01"),
("3e6d4400-662c-4f41-8fd7-58a6f389a118", "Book 19", "Author 19", "Publisher 19", 190, "2023-07-01"),
("d98ef9de-ec20-4c1a-8d25-4498b4051b7e", "Book 20", "Author 20", "Publisher 20", 220, "2023-08-01");

INSERT INTO iletter_db.book_lot (
    book_isbn, price_per_unit, available_units, inventory_id
)
VALUES
("e51c0a75-cc5f-4d10-bac6-12e1b1f7699a", 10.99, 5, 1),
("04385f6a-9724-4c71-9e5d-6de2396639d2", 10.99, 10, 2),
("7be6f607-4812-4f32-a141-3e0dd88850f7", 10.99, 15, 3),
("2a8270ce-961d-46b1-9f2e-bd6de4f60f2a", 10.99, 20, 4),
("c9964fc6-9cb4-4d5e-b4b2-7bc7dc9a9426", 12.99, 8, 1),
("fb0c2b61-7a8f-4a68-809f-6467802b0646", 12.99, 12, 2),
("a392ff1d-3f6c-41e4-8e11-f496d9777704", 12.99, 16, 3),
("75992561-47a3-4a37-87d1-d8eb5d7a716a", 12.99, 20, 4),
("9a181dca-8459-4fa5-a7ed-836a7d747536", 8.99, 3, 1),
("e6e3999a-833b-43b5-936d-879790af0a7d", 8.99, 6, 2),
("6820d5f7-615b-4a22-bdce-6517f2f822f2", 8.99, 9, 3),
("b5bdcf09-8c45-4c99-a2d9-0f692c74b396", 8.99, 12, 4),
("15b9eef3-2494-4a3e-94b4-493be2a08ce4", 15.99, 4, 1),
("25d10384-8d4c-4d4e-a4b1-6e0ef06b6280", 15.99, 8, 2),
("c550b737-64d1-4fd2-af0f-57a77ef27f33", 15.99, 12, 3),
("c6e8fb3b-8765-4e6f-aa42-ef8a89e0a474", 15.99, 16, 4),
("d31294e5-45dd-4be3-a26a-9b8c90f42c32", 9.99, 6, 1),
("85f06d70-12bc-4d68-a8c5-fb392482f80e", 9.99, 12, 2),
("3e6d4400-662c-4f41-8fd7-58a6f389a118", 11.99, 28, 4),
("d98ef9de-ec20-4c1a-8d25-4498b4051b7e", 13.99, 9, 1);

INSERT INTO iletter_db.branch_office (inventory_id, manager_id) VALUES
(1, 1), (2, 1), (3, 2), (4, 2), (1, 3), (2, 3), (3, 4), (4, 4), (1, 5), (2, 5),
(3, 6), (4, 6), (1, 7), (2, 7), (3, 8), (4, 8), (1, 9), (2, 9), (3, 10),
(4, 10);

INSERT INTO iletter_db.worker (
    first_name, middle_name, first_last_name, second_last_name, identification,
    date_of_birth, email_address, phone_number, residence_address, date_of_hire,
    role, basic_salary, branch_office_id
) VALUES
("John", "Doe", "Smith", "Doe", "1234567890", "1990-01-01", "john.doe@example.com", "1234567890", "123 Main Street", "2020-01-01", "Manager", 5000.00, 1),
("Jane", "Doe", "Smith", "Doe", "0987654321", "1995-02-01", "jane.doe@example.com", "0987654321", "456 Elm Street", "2020-02-01", "Supervisor", 4000.00, 2),
("Michael", "Johnson", "Brown", "Johnson", "5678901234", "1988-03-01", "michael.johnson@example.com", "5678901234", "789 Oak Street", "2020-03-01", "Employee", 3000.00, 1),
("Emily", "Williams", "Taylor", "Williams", "4321098765", "1992-04-01", "emily.williams@example.com", "4321098765", "321 Pine Street", "2020-04-01", "Employee", 3000.00, 2),
("David", "Miller", "Anderson", "Miller", "9876543210", "1991-05-01", "david.miller@example.com", "9876543210", "654 Cedar Street", "2020-05-01", "Employee", 3000.00, 1),
("Sarah", "Wilson", "Harris", "Wilson", "3456789012", "1993-06-01", "sarah.wilson@example.com", "3456789012", "987 Maple Street", "2020-06-01", "Employee", 3000.00, 2),
("Christopher", "Jones", "Thompson", "Jones", "8765432109", "1989-07-01", "christopher.jones@example.com", "8765432109", "789 Oak Street", "2020-07-01", "Employee", 3000.00, 1),
("Olivia", "Anderson", "Davis", "Anderson", "2109876543", "1994-08-01", "olivia.anderson@example.com", "2109876543", "654 Cedar Street", "2020-08-01", "Employee", 3000.00, 2),
("Daniel", "Martinez", "Brown", "Martinez", "7654321098", "1990-09-01", "daniel.martinez@example.com", "7654321098", "321 Pine Street", "2020-09-01", "Employee", 3000.00, 1),
("Sophia", "Garcia", "Taylor", "Garcia", "1098765432", "1996-10-01", "sophia.garcia@example.com", "1098765432", "987 Maple Street", "2020-10-01", "Employee", 3000.00, 2),
("Ella", "Thomas", "Davis", "Thomas", "6543210987", "1995-11-01", "ella.thomas@example.com", "6543210987", "123 Main Street", "2020-11-01", "Employee", 3000.00, 1),
("Benjamin", "Hernandez", "Brown", "Hernandez", "5432109876", "1992-12-01", "benjamin.hernandez@example.com", "5432109876", "456 Elm Street", "2020-12-01", "Employee", 3000.00, 2),
("Mia", "Gonzalez", "Taylor", "Gonzalez", "4321098765", "1993-01-01", "mia.gonzalez@example.com", "4321098765", "789 Oak Street", "2021-01-01", "Employee", 3000.00, 1),
("Henry", "Wilson", "Harris", "Wilson", "3210987654", "1991-02-01", "henry.wilson@example.com", "3210987654", "321 Pine Street", "2021-02-01", "Employee", 3000.00, 2),
("Victoria", "Lee", "Thompson", "Lee", "2109876543", "1994-03-01", "victoria.lee@example.com", "2109876543", "654 Cedar Street", "2021-03-01", "Employee", 3000.00, 1),
("Sebastian", "Walker", "Anderson", "Walker", "1098765432", "1990-04-01", "sebastian.walker@example.com", "1098765432", "987 Maple Street", "2021-04-01", "Employee", 3000.00, 2),
("Ava", "Gomez", "Brown", "Gomez", "0987654321", "1996-05-01", "ava.gomez@example.com", "0987654321", "123 Main Street", "2021-05-01", "Employee", 3000.00, 1),
("Gabriel", "Perez", "Taylor", "Perez", "9876543210", "1989-06-01", "gabriel.perez@example.com", "9876543210", "456 Elm Street", "2021-06-01", "Employee", 3000.00, 2),
("Sophie", "Sanchez", "Harris", "Sanchez", "8765432109", "1992-07-01", "sophie.sanchez@example.com", "8765432109", "789 Oak Street", "2021-07-01", "Employee", 3000.00, 1),
("Jackson", "Rivera", "Thompson", "Rivera", "7654321098", "1996-08-01", "jackson.rivera@example.com", "7654321098", "321 Pine Street", "2021-08-01", "Employee", 3000.00, 2);

INSERT INTO iletter_db.client (
    first_name, middle_name, first_last_name, second_last_name, identification,
    email_address, phone_number
) VALUES
("John", "Doe", "Smith", "Doe", "1234567890", "john.doe@example.com", "1234567890"),
("Jane", "Doe", "Smith", "Doe", "0987654321", "jane.doe@example.com", "0987654321"),
("Michael", "Johnson", "Brown", "Johnson", "5678901234", "michael.johnson@example.com", "5678901234"),
("Emily", "Williams", "Taylor", "Williams", "4321098765", "emily.williams@example.com", "4321098765"),
("David", "Miller", "Anderson", "Miller", "9876543210", "david.miller@example.com", "9876543210"),
("Sarah", "Wilson", "Harris", "Wilson", "3456789012", "sarah.wilson@example.com", "3456789012"),
("Christopher", "Jones", "Thompson", "Jones", "8765432109", "christopher.jones@example.com", "8765432109"),
("Olivia", "Anderson", "Davis", "Anderson", "2109876543", "olivia.anderson@example.com", "2109876543"),
("Daniel", "Martinez", "Brown", "Martinez", "7654321098", "daniel.martinez@example.com", "7654321098"),
("Sophia", "Garcia", "Taylor", "Garcia", "1098765432", "sophia.garcia@example.com", "1098765432"),
("Matthew", "Lopez", "Harris", "Lopez", "2345678901", "matthew.lopez@example.com", "2345678901"),
("Amelia", "Clark", "Miller", "Clark", "4567890123", "amelia.clark@example.com", "4567890123"),
("James", "Young", "Adams", "Young", "7890123456", "james.young@example.com", "7890123456"),
("Ava", "Walker", "Edwards", "Walker", "5678901234", "ava.walker@example.com", "5678901234"),
("Liam", "Hall", "Collins", "Hall", "9012345678", "liam.hall@example.com", "9012345678"),
("Emma", "Green", "Murphy", "Green", "4321098765", "emma.green@example.com", "4321098765"),
("Noah", "Turner", "Clark", "Turner", "8901234567", "noah.turner@example.com", "8901234567"),
("Mia", "Parker", "Baker", "Parker", "7654321098", "mia.parker@example.com", "7654321098"),
("William", "White", "Cook", "White", "2109876543", "william.white@example.com", "2109876543"),
("Isabella", "King", "Bell", "King", "5432109876", "isabella.king@example.com", "5432109876");

INSERT INTO iletter_db.branch_office_clients (
    client_id, branch_office_id
) VALUES
(1, 1), (2, 1), (3, 1), (4, 2), (5, 2), (6, 2), (7, 3), (8, 3), (9, 3), (10, 4),
(11, 4), (12, 4), (13, 5), (14, 5), (15, 5), (16, 6), (17, 6), (18, 6), (19, 7),
(20, 7);

INSERT INTO iletter_db.order_details (book_name, quantity, total_price) VALUES
("Book 1", 2, 25.99), ("Book 2", 1, 12.50), ("Book 3", 3, 36.75),
("Book 4", 1, 9.99), ("Book 5", 2, 19.98), ("Book 6", 1, 15.99),
("Book 7", 4, 49.95), ("Book 8", 3, 39.75), ("Book 9", 2, 23.50),
("Book 10", 1, 8.99), ("Book 11", 2, 29.98), ("Book 12", 1, 17.99),
("Book 13", 3, 34.50), ("Book 14", 1, 11.99), ("Book 15", 2, 21.98),
("Book 16", 4, 47.95), ("Book 17", 3, 37.75), ("Book 18", 2, 25.50),
("Book 19", 1, 7.99), ("Book 20", 2, 27.98);

INSERT INTO iletter_db.order (
    branch_office_id, order_details_id, client_id
) VALUES
(1, 1, 1), (1, 2, 2), (2, 3, 3), (2, 4, 4), (3, 5, 5), (3, 6, 6), (4, 7, 7),
(4, 8, 8), (5, 9, 9), (5, 10, 10), (6, 11, 11), (6, 12, 12), (7, 13, 13),
(7, 14, 14), (8, 15, 15), (8, 16, 16), (9, 17, 17), (9, 18, 18), (10, 19, 19),
(10, 20, 20);

INSERT INTO iletter_db.administrator_details (
    first_name, middle_name, first_last_name, second_last_name, identification,
    email_address, phone_number, branch_office_id
) VALUES
("John", "Doe", "Smith", "Johnson", "1234567890", "john.doe@example.com", "1234567890", 1),
("Jane", "Doe", "Smith", "Johnson", "0987654321", "jane.doe@example.com", "9876543210", 2),
("Michael", "Smith", "Johnson", "Doe", "4567890123", "michael.smith@example.com", "4567890123", 1),
("Emily", "Smith", "Johnson", "Doe", "9876543210", "emily.smith@example.com", "9876543210", 2),
("William", "Johnson", "Doe", "Smith", "2345678901", "william.johnson@example.com", "2345678901", 3),
("Olivia", "Johnson", "Doe", "Smith", "8765432109", "olivia.johnson@example.com", "8765432109", 4),
("James", "Doe", "Smith", "Johnson", "3456789012", "james.doe@example.com", "3456789012", 3),
("Sophia", "Doe", "Smith", "Johnson", "7654321098", "sophia.doe@example.com", "7654321098", 4),
("Benjamin", "Smith", "Johnson", "Doe", "4567890123", "benjamin.smith@example.com", "4567890123", 5),
("Ava", "Smith", "Johnson", "Doe", "6543210987", "ava.smith@example.com", "6543210987", 6),
("Jacob", "Johnson", "Doe", "Smith", "5678901234", "jacob.johnson@example.com", "5678901234", 5),
("Mia", "Johnson", "Doe", "Smith", "5432109876", "mia.johnson@example.com", "5432109876", 6),
("Ethan", "Doe", "Smith", "Johnson", "6789012345", "ethan.doe@example.com", "6789012345", 7),
("Isabella", "Doe", "Smith", "Johnson", "4321098765", "isabella.doe@example.com", "4321098765", 7),
("Alexander", "Smith", "Johnson", "Doe", "7890123456", "alexander.smith@example.com", "7890123456", 8),
("Charlotte", "Smith", "Johnson", "Doe", "3210987654", "charlotte.smith@example.com", "3210987654", 8),
("Daniel", "Johnson", "Doe", "Smith", "8901234567", "daniel.johnson@example.com", "8901234567", 9),
("Amelia", "Johnson", "Doe", "Smith", "2109876543", "amelia.johnson@example.com", "2109876543", 9),
("Liam", "Smith", "Johnson", "Doe", "1234567890", "liam.smith@example.com", "1234567890", 10),
("Lily", "Smith", "Johnson", "Doe", "0987654321", "lily.smith@example.com", "9876543210", 10);

INSERT INTO iletter_db.administrator (
    user_name, password_hash, administrator_details_id
) VALUES
("admin1", "password1", 1), ("admin2", "password2", 2),
("admin3", "password3", 3), ("admin4", "password4", 4),
("admin5", "password5", 5), ("admin6", "password6", 6),
("admin7", "password7", 7), ("admin8", "password8", 8),
("admin9", "password9", 9), ("admin10", "password10", 10),
("admin11", "password11", 11), ("admin12", "password12", 12),
("admin13", "password13", 13), ("admin14", "password14", 14),
("admin15", "password15", 15), ("admin16", "password16", 16),
("admin17", "password17", 17), ("admin18", "password18", 18),
("admin19", "password19", 19), ("admin20", "password20", 20);

INSERT INTO iletter_db.process_audit (manipulation_type, date_time) VALUES
('CREATE', '2023-05-01 10:00:00'), ('UPDATE', '2023-05-02 11:30:00'),
('DELETE', '2023-05-03 14:45:00'), ('UPDATE', '2023-05-04 09:15:00'),
('CREATE', '2023-05-05 12:20:00'), ('DELETE', '2023-05-06 16:10:00'),
('UPDATE', '2023-05-07 13:25:00'), ('CREATE', '2023-05-08 10:30:00'),
('DELETE', '2023-05-09 15:40:00'), ('UPDATE', '2023-05-10 11:50:00'),
('CREATE', '2023-05-11 14:15:00'), ('DELETE', '2023-05-12 09:55:00'),
('UPDATE', '2023-05-13 12:05:00'), ('CREATE', '2023-05-14 15:30:00'),
('DELETE', '2023-05-15 10:20:00'), ('UPDATE', '2023-05-16 13:40:00'),
('CREATE', '2023-05-17 11:10:00'), ('DELETE', '2023-05-18 14:50:00'),
('UPDATE', '2023-05-19 09:25:00'), ('CREATE', '2023-05-20 12:35:00');

INSERT INTO iletter_db.process_audit_administrators (
    process_audit_id, administrator_id
) VALUES
    (1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6), (7, 7), (8, 8), (9, 9),
    (10, 10), (11, 11), (12, 12), (13, 13), (14, 14), (15, 15), (16, 16),
    (17, 17), (18, 18), (19, 19), (20, 20);
