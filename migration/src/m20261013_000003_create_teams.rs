use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Adjacency list: the CHECK only blocks direct self-parenting.
        // Longer cycles must be prevented by the application.
        manager
            .create_table(
                Table::create()
                    .table(Teams::Table)
                    .col(pk_uuid(Teams::Id).default(Expr::cust("gen_random_uuid()")))
                    .col(
                        text(Teams::Name)
                            .check(("teams_name_not_blank", Expr::cust("btrim(name) <> ''"))),
                    )
                    .col(text_null(Teams::Description))
                    .col(uuid_null(Teams::ParentTeamId))
                    .col(timestamp_with_time_zone_default_now(Teams::CreatedAt))
                    .col(timestamp_with_time_zone_default_now(Teams::UpdatedAt))
                    .check((
                        "teams_no_self_parent",
                        Expr::cust("parent_team_id IS NULL OR parent_team_id <> id"),
                    ))
                    .foreign_key(
                        ForeignKey::create()
                            .name("teams_parent_team_id_fkey")
                            .from(Teams::Table, Teams::ParentTeamId)
                            .to(Teams::Table, Teams::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("teams_parent_team_idx")
                    .table(Teams::Table)
                    .col(Teams::ParentTeamId)
                    .to_owned(),
            )
            .await?;

        // PK(user_id) enforces at most one team per user.
        manager
            .create_table(
                Table::create()
                    .table(TeamMembers::Table)
                    .col(uuid(TeamMembers::UserId).primary_key())
                    .col(uuid(TeamMembers::TeamId))
                    .col(timestamp_with_time_zone_default_now(TeamMembers::JoinedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("team_members_user_id_fkey")
                            .from(TeamMembers::Table, TeamMembers::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("team_members_team_id_fkey")
                            .from(TeamMembers::Table, TeamMembers::TeamId)
                            .to(Teams::Table, Teams::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("team_members_team_id_idx")
                    .table(TeamMembers::Table)
                    .col(TeamMembers::TeamId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TeamMembers::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Teams::Table).to_owned())
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
enum Teams {
    Table,
    Id,
    Name,
    Description,
    ParentTeamId,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum TeamMembers {
    Table,
    UserId,
    TeamId,
    JoinedAt,
}
