use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Users are linked by (provider, issuer, subject), never by matching email.
        manager
            .create_table(
                Table::create()
                    .table(OauthIdentities::Table)
                    .col(pk_uuid(OauthIdentities::Id).default(Expr::cust("gen_random_uuid()")))
                    .col(uuid(OauthIdentities::UserId))
                    .col(
                        text(OauthIdentities::Provider).check(Expr::cust("provider = 'microsoft'")),
                    )
                    .col(
                        text(OauthIdentities::Issuer)
                            .check(("oauth_issuer_not_blank", Expr::cust("btrim(issuer) <> ''"))),
                    )
                    .col(text(OauthIdentities::Subject).check((
                        "oauth_subject_not_blank",
                        Expr::cust("btrim(subject) <> ''"),
                    )))
                    .col(timestamp_with_time_zone_default_now(
                        OauthIdentities::CreatedAt,
                    ))
                    .foreign_key(
                        ForeignKey::create()
                            .name("oauth_identities_user_id_fkey")
                            .from(OauthIdentities::Table, OauthIdentities::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("oauth_identity_unique")
                    .table(OauthIdentities::Table)
                    .unique()
                    .col(OauthIdentities::Provider)
                    .col(OauthIdentities::Issuer)
                    .col(OauthIdentities::Subject)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("oauth_identities_user_id_idx")
                    .table(OauthIdentities::Table)
                    .col(OauthIdentities::UserId)
                    .to_owned(),
            )
            .await?;

        // Refresh tokens are stored hashed, never in plaintext.
        manager
            .create_table(
                Table::create()
                    .table(AuthSessions::Table)
                    .col(pk_uuid(AuthSessions::Id).default(Expr::cust("gen_random_uuid()")))
                    .col(uuid(AuthSessions::UserId))
                    .col(text_uniq(AuthSessions::RefreshTokenHash))
                    .col(timestamp_with_time_zone_default_now(
                        AuthSessions::CreatedAt,
                    ))
                    .col(timestamp_with_time_zone(AuthSessions::ExpiresAt))
                    .col(timestamp_with_time_zone_null(AuthSessions::RevokedAt))
                    .check((
                        "auth_sessions_expiration",
                        Expr::cust("expires_at > created_at"),
                    ))
                    .check((
                        "auth_sessions_revocation",
                        Expr::cust("revoked_at IS NULL OR revoked_at >= created_at"),
                    ))
                    .foreign_key(
                        ForeignKey::create()
                            .name("auth_sessions_user_id_fkey")
                            .from(AuthSessions::Table, AuthSessions::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("auth_sessions_user_id_idx")
                    .table(AuthSessions::Table)
                    .col(AuthSessions::UserId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AuthSessions::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(OauthIdentities::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum OauthIdentities {
    Table,
    Id,
    UserId,
    Provider,
    Issuer,
    Subject,
    CreatedAt,
}

#[derive(DeriveIden)]
enum AuthSessions {
    Table,
    Id,
    UserId,
    RefreshTokenHash,
    CreatedAt,
    ExpiresAt,
    RevokedAt,
}
