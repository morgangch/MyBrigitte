use sea_orm::entity::prelude::*;

/// An external account (Microsoft) linked to a user, found back by `(provider, issuer, subject)` at sign-in.
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "oauth_identities")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub user_id: Uuid,
    pub provider: String,
    pub issuer: String,
    pub subject: String,
    pub created_at: DateTimeWithTimeZone,
    #[sea_orm(belongs_to, from = "user_id", to = "id")]
    pub user: BelongsTo<super::user::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
