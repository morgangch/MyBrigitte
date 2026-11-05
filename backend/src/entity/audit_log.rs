use sea_orm::entity::prelude::*;

/// Stored as `text` with a `CHECK`, not as a Postgres enum.
#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Text")]
pub enum Action {
    #[sea_orm(string_value = "CREATE")]
    Create,
    #[sea_orm(string_value = "UPDATE")]
    Update,
    #[sea_orm(string_value = "DELETE")]
    Delete,
}

/// Append-only history of changes to any table: rows are inserted, never updated or deleted.
/// `performed_by` is `None` for changes made by the system.
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "audit_log")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub table_name: String,
    pub record_id: Uuid,
    pub action: Action,
    pub event_type: Option<String>,
    pub old_data: Option<Json>,
    pub new_data: Option<Json>,
    pub performed_by: Option<Uuid>,
    pub reason: Option<String>,
    pub created_at: DateTimeWithTimeZone,
    #[sea_orm(belongs_to, from = "performed_by", to = "id")]
    pub performer: BelongsTo<Option<super::user::Entity>>,
}

impl ActiveModelBehavior for ActiveModel {}
