use sea_orm_migration::{prelude::extension::postgres::Type, prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Closed set: fixed weekly hours, or a flexible weekly target.
        manager
            .create_type(
                Type::create()
                    .as_enum(WorkPolicyMode::Mode)
                    .values([WorkPolicyMode::Fixed, WorkPolicyMode::Flexible])
                    .to_owned(),
            )
            .await?;

        // Validity periods are [effective_from, effective_until), upper bound exclusive.
        manager
            .create_table(
                Table::create()
                    .table(UserWorkPolicies::Table)
                    .col(pk_uuid(UserWorkPolicies::Id).default(Expr::cust("gen_random_uuid()")))
                    .col(uuid(UserWorkPolicies::UserId))
                    .col(enumeration(
                        UserWorkPolicies::Mode,
                        WorkPolicyMode::Mode,
                        [WorkPolicyMode::Fixed, WorkPolicyMode::Flexible],
                    ))
                    .col(
                        integer_null(UserWorkPolicies::ExpectedMinutesPerWeek).check(Expr::cust(
                            "expected_minutes_per_week IS NULL OR expected_minutes_per_week >= 0",
                        )),
                    )
                    .col(
                        small_integer(UserWorkPolicies::DefaultBreakMinutes)
                            .default(0)
                            .check(Expr::cust("default_break_minutes >= 0")),
                    )
                    .col(
                        small_integer(UserWorkPolicies::LateToleranceMinutes)
                            .default(5)
                            .check(Expr::cust("late_tolerance_minutes >= 0")),
                    )
                    .col(date(UserWorkPolicies::EffectiveFrom))
                    .col(date_null(UserWorkPolicies::EffectiveUntil))
                    .col(uuid(UserWorkPolicies::CreatedBy))
                    .col(timestamp_with_time_zone_default_now(
                        UserWorkPolicies::CreatedAt,
                    ))
                    .check((
                        "policies_positive_period",
                        Expr::cust("effective_until IS NULL OR effective_until > effective_from"),
                    ))
                    .foreign_key(
                        ForeignKey::create()
                            .name("user_work_policies_user_id_fkey")
                            .from(UserWorkPolicies::Table, UserWorkPolicies::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("user_work_policies_created_by_fkey")
                            .from(UserWorkPolicies::Table, UserWorkPolicies::CreatedBy)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                "ALTER TABLE user_work_policies
                    ADD CONSTRAINT policies_no_overlap EXCLUDE USING gist (
                        user_id WITH =,
                        daterange(effective_from, effective_until, '[)') WITH &&
                    )",
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("user_work_policies_user_idx")
                    .table(UserWorkPolicies::Table)
                    .col(UserWorkPolicies::UserId)
                    .to_owned(),
            )
            .await?;

        // One row = one expected day (ISO day_of_week, Monday=1), not one session.
        // Both times NULL means "no work expected" for that day.
        manager
            .create_table(
                Table::create()
                    .table(WorkSchedules::Table)
                    .col(pk_uuid(WorkSchedules::Id).default(Expr::cust("gen_random_uuid()")))
                    .col(uuid(WorkSchedules::PolicyId))
                    .col(
                        small_integer(WorkSchedules::DayOfWeek)
                            .check(Expr::cust("day_of_week BETWEEN 1 AND 7")),
                    )
                    .col(date(WorkSchedules::ValidFrom))
                    .col(date_null(WorkSchedules::ValidUntil))
                    .col(time_null(WorkSchedules::StartTime))
                    .col(time_null(WorkSchedules::EndTime))
                    .col(
                        small_integer(WorkSchedules::ExpectedBreakMinutes)
                            .default(0)
                            .check(Expr::cust("expected_break_minutes >= 0")),
                    )
                    .col(timestamp_with_time_zone_default_now(WorkSchedules::CreatedAt))
                    .check((
                        "schedules_positive_period",
                        Expr::cust("valid_until IS NULL OR valid_until > valid_from"),
                    ))
                    .check((
                        "schedules_times_both_or_neither",
                        Expr::cust(
                            "(start_time IS NULL AND end_time IS NULL AND expected_break_minutes = 0)
                            OR (start_time IS NOT NULL AND end_time IS NOT NULL AND start_time <> end_time)",
                        ),
                    ))
                    // Day envelopes are < 24h: an end earlier than the start means next day.
                    .check((
                        "schedules_break_shorter_than_envelope",
                        Expr::cust(
                            "start_time IS NULL OR expected_break_minutes <
                            (CASE WHEN end_time > start_time
                                  THEN EXTRACT(EPOCH FROM (end_time - start_time)) / 60
                                  ELSE (86400 + EXTRACT(EPOCH FROM (end_time - start_time))) / 60
                             END)",
                        ),
                    ))
                    .foreign_key(
                        ForeignKey::create()
                            .name("work_schedules_policy_id_fkey")
                            .from(WorkSchedules::Table, WorkSchedules::PolicyId)
                            .to(UserWorkPolicies::Table, UserWorkPolicies::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("work_schedules_lookup_idx")
                    .table(WorkSchedules::Table)
                    .col(WorkSchedules::PolicyId)
                    .col(WorkSchedules::DayOfWeek)
                    .col(WorkSchedules::ValidFrom)
                    .col((WorkSchedules::CreatedAt, IndexOrder::Desc))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(WorkSchedules::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(UserWorkPolicies::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(WorkPolicyMode::Mode).to_owned())
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
enum WorkPolicyMode {
    #[sea_orm(iden = "work_policy_mode")]
    Mode,
    #[sea_orm(iden = "fixed")]
    Fixed,
    #[sea_orm(iden = "flexible")]
    Flexible,
}

#[derive(DeriveIden)]
enum UserWorkPolicies {
    Table,
    Id,
    UserId,
    Mode,
    ExpectedMinutesPerWeek,
    DefaultBreakMinutes,
    LateToleranceMinutes,
    EffectiveFrom,
    EffectiveUntil,
    CreatedBy,
    CreatedAt,
}

#[derive(DeriveIden)]
enum WorkSchedules {
    Table,
    Id,
    PolicyId,
    DayOfWeek,
    ValidFrom,
    ValidUntil,
    StartTime,
    EndTime,
    ExpectedBreakMinutes,
    CreatedAt,
}
