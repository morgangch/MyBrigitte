use sea_orm::entity::prelude::*;

/// A user's single team. `user_id` is the primary key, so a user belongs to at most one team.
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "team_members")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: Uuid,
    pub team_id: Uuid,
    pub joined_at: DateTimeWithTimeZone,
}

impl ActiveModelBehavior for ActiveModel {}
