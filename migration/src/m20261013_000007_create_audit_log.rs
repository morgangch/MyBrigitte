use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Append-only: the runtime DB role should not be granted UPDATE or DELETE here.
        manager
            .create_table(
                Table::create()
                    .table(AuditLog::Table)
                    .col(pk_uuid(AuditLog::Id).default(Expr::cust("gen_random_uuid()")))
                    .col(text(AuditLog::TableName))
                    .col(uuid(AuditLog::RecordId))
                    .col(
                        text(AuditLog::Action)
                            .check(Expr::cust("action IN ('CREATE', 'UPDATE', 'DELETE')")),
                    )
                    .col(text_null(AuditLog::EventType))
                    .col(json_binary_null(AuditLog::OldData))
                    .col(json_binary_null(AuditLog::NewData))
                    .col(uuid_null(AuditLog::PerformedBy))
                    .col(text_null(AuditLog::Reason))
                    .col(timestamp_with_time_zone_default_now(AuditLog::CreatedAt))
                    .check((
                        "audit_log_correction_reason",
                        Expr::cust(
                            "event_type IS DISTINCT FROM 'WORK_SESSION_CORRECTED'
                            OR (reason IS NOT NULL AND btrim(reason) <> '')",
                        ),
                    ))
                    .foreign_key(
                        ForeignKey::create()
                            .name("audit_log_performed_by_fkey")
                            .from(AuditLog::Table, AuditLog::PerformedBy)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("audit_log_record_idx")
                    .table(AuditLog::Table)
                    .col(AuditLog::TableName)
                    .col(AuditLog::RecordId)
                    .col((AuditLog::CreatedAt, IndexOrder::Desc))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("audit_log_actor_idx")
                    .table(AuditLog::Table)
                    .col(AuditLog::PerformedBy)
                    .col((AuditLog::CreatedAt, IndexOrder::Desc))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AuditLog::Table).to_owned())
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
enum AuditLog {
    Table,
    Id,
    TableName,
    RecordId,
    Action,
    EventType,
    OldData,
    NewData,
    PerformedBy,
    Reason,
    CreatedAt,
}
