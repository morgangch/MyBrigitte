use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "work_policy_mode")]
pub enum Mode {
    #[sea_orm(string_value = "fixed")]
    Fixed,
    #[sea_orm(string_value = "flexible")]
    Flexible,
}

/// A user's working-time rules over [effective_from, effective_until).
/// A `None` effective_until means the policy is still in force.
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user_work_policies")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub user_id: Uuid,
    pub mode: Mode,
    pub expected_minutes_per_week: Option<i32>,
    pub default_break_minutes: i16,
    pub late_tolerance_minutes: i16,
    pub effective_from: Date,
    pub effective_until: Option<Date>,
    pub created_by: Uuid,
    pub created_at: DateTimeWithTimeZone,
    // Two foreign keys to `users`: each relation needs its own name.
    #[sea_orm(belongs_to, relation_enum = "User", from = "user_id", to = "id")]
    pub user: BelongsTo<super::user::Entity>,
    #[sea_orm(belongs_to, relation_enum = "Creator", from = "created_by", to = "id")]
    pub creator: BelongsTo<super::user::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

// Two relations reach `users`, so SeaORM does not generate `Related`: the owner is the default.
// Reach the author with a join on `Relation::Creator`.
impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}
