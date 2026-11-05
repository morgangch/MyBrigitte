use sea_orm::entity::prelude::*;

/// One expected day of a policy (ISO `day_of_week`, Monday = 1), not one session.
/// `None` start and end times mean no work is expected that day.
/// An `end_time` earlier than `start_time` ends the next day.
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "work_schedules")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub policy_id: Uuid,
    pub day_of_week: i16,
    pub valid_from: Date,
    pub valid_until: Option<Date>,
    pub start_time: Option<Time>,
    pub end_time: Option<Time>,
    pub expected_break_minutes: i16,
    pub created_at: DateTimeWithTimeZone,
    #[sea_orm(belongs_to, from = "policy_id", to = "id")]
    pub policy: BelongsTo<super::user_work_policy::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
