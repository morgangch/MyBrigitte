use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // A NULL end_time is an open session.
        manager
            .create_table(
                Table::create()
                    .table(WorkSessions::Table)
                    .col(pk_uuid(WorkSessions::Id).default(Expr::cust("gen_random_uuid()")))
                    .col(uuid(WorkSessions::UserId))
                    .col(timestamp_with_time_zone(WorkSessions::StartTime))
                    .col(timestamp_with_time_zone_null(WorkSessions::EndTime))
                    .col(timestamp_with_time_zone_default_now(
                        WorkSessions::CreatedAt,
                    ))
                    .col(timestamp_with_time_zone_default_now(
                        WorkSessions::UpdatedAt,
                    ))
                    .check((
                        "work_sessions_valid_interval",
                        Expr::cust("end_time IS NULL OR end_time > start_time"),
                    ))
                    .foreign_key(
                        ForeignKey::create()
                            .name("work_sessions_user_id_fkey")
                            .from(WorkSessions::Table, WorkSessions::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;

        // Also rejects overlap against an open-ended session.
        manager
            .get_connection()
            .execute_unprepared(
                "ALTER TABLE work_sessions
                    ADD CONSTRAINT work_sessions_no_overlap EXCLUDE USING gist (
                        user_id WITH =,
                        tstzrange(start_time, end_time, '[)') WITH &&
                    )",
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("work_sessions_one_open_per_user")
                    .table(WorkSessions::Table)
                    .unique()
                    .col(WorkSessions::UserId)
                    .and_where(Expr::col(WorkSessions::EndTime).is_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("work_sessions_user_start_idx")
                    .table(WorkSessions::Table)
                    .col(WorkSessions::UserId)
                    .col((WorkSessions::StartTime, IndexOrder::Desc))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(WorkSessions::Table).to_owned())
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
enum WorkSessions {
    Table,
    Id,
    UserId,
    StartTime,
    EndTime,
    CreatedAt,
    UpdatedAt,
}
