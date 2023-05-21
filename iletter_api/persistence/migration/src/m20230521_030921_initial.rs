use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // manager
        //     .create_table(
        //         Table::create()
        //             .table(Post::Table)
        //             .if_not_exists()
        //             .col(
        //                 ColumnDef::new(Post::Id)
        //                     .integer()
        //                     .not_null()
        //                     .auto_increment()
        //                     .primary_key(),
        //             )
        //             .col(ColumnDef::new(Post::Title).string().not_null())
        //             .col(ColumnDef::new(Post::Text).string().not_null())
        //             .to_owned(),
        //     )
        //     .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}

// Tables
#[derive(Iden)]
enum Manager {
    Table,
    Id,
    FirstName,
    MiddleName,
    FirstLastName,
    SecondLastName,
    Identification,
    DateOfBirth,
    EmailAddress,
    PhoneNumber,
    ResidenceAddress,
    DateOfHire,
    BasicSalary,
}

#[derive(Iden)]
enum Inventory {
    Table,
    Id,
    Description,
}

#[derive(Iden)]
enum Book {
    Table,
    Isbn,
    Name,
    AuthorName,
    PublisherName,
    NumberOfPages,
    PublicationDate,
}

#[derive(Iden)]
enum BookLot {
    Table,
    BookIsbn,
    PricePerUnit,
    AvailableUnits,
    InventoryId,
}

#[derive(Iden)]
enum BranchOffice {
    Table,
    Id,
    InventoryId,
    ManagerId,
}

#[derive(Iden)]
enum Worker {
    Table,
    Id,
    FirstName,
    MiddleName,
    FirstLastName,
    SecondLastName,
    Identification,
    DateOfBirth,
    EmailAddress,
    PhoneNumber,
    ResidenceAddress,
    DateOfHire,
    Role,
    BasicSalary,
    BranchOfficeId,
}

#[derive(Iden)]
enum Client {
    Table,
    Id,
    FirstName,
    MiddleName,
    FirstLastName,
    SecondLastName,
    Identification,
    DateOfBirth,
    EmailAddress,
    PhoneNumber,
}

#[derive(Iden)]
enum BranchOfficeClients {
    Table,
    ClientId,
    BranchOfficeId,
}

#[derive(Iden)]
enum OrderDetails {
    Table,
    Id,
    BookName,
    Quantity,
    TotalPrice,
}

#[derive(Iden)]
enum Order {
    Table,
    Id,
    BranchOfficeId,
    OrderDetailsId,
    ClientId,
}

#[derive(Iden)]
enum AdministratorDetails {
    Table,
    Id,
    FirstName,
    MiddleName,
    FirstLastName,
    SecondLastName,
    Identification,
    DateOfBirth,
    EmailAddress,
    PhoneNumber,
    BranchOfficeId,
}

#[derive(Iden)]
enum Administrator {
    Table,
    Id,
    UserName,
    PasswordHash,
    AdministratorDetailsId,
}

#[derive(Iden)]
enum ProcessAudit {
    Table,
    Id,
    ManipulationType,
    DateTime,
}

#[derive(Iden)]
enum ProcessAuditAdministrators {
    ProcessAuditId,
    AdministratorId,
}
