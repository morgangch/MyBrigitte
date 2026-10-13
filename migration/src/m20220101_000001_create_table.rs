use sea_orm_migration::{prelude::extension::postgres::Type, prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(UserRole::Type)
                    .values([UserRole::Employee, UserRole::Manager])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(pk_uuid(Users::Id).default(Expr::cust("gen_random_uuid()")))
                    .col(string_len(Users::FirstName, 100))
                    .col(string_len(Users::LastName, 100))
                    .col(
                        string_len(Users::Email, 254)
                            .check(("users_email_not_blank", Expr::cust("btrim(email) <> ''"))),
                    )
                    .col(string_len_null(Users::PhoneNumber, 20))
                    .col(string_len_null(Users::PasswordHash, 255))
                    .col(enumeration(
                        Users::Role,
                        UserRole::Type,
                        [UserRole::Employee, UserRole::Manager],
                    ))
                    .col(boolean(Users::IsActive).default(true))
                    .col(timestamp_with_time_zone_default_now(Users::CreatedAt))
                    .col(timestamp_with_time_zone_default_now(Users::UpdatedAt))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("users_email_lower_unique")
                    .table(Users::Table)
                    .unique()
                    .col(Func::lower(Expr::col(Users::Email)))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(UserRole::Type).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    FirstName,
    LastName,
    Email,
    PhoneNumber,
    PasswordHash,
    Role,
    IsActive,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum UserRole {
    #[sea_orm(iden = "user_role")]
    Type,
    #[sea_orm(iden = "employee")]
    Employee,
    #[sea_orm(iden = "manager")]
    Manager,
}
