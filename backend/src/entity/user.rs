use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "user_role")]
pub enum Role {
    #[sea_orm(string_value = "employee")]
    Employee,
    #[sea_orm(string_value = "manager")]
    Manager,
}

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub password_hash: Option<String>,
    pub role: Role,
    pub is_active: bool,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(has_many)]
    pub work_sessions: HasMany<super::work_session::Entity>,
    #[sea_orm(has_many, relation_enum = "WorkPolicies", via_rel = "User")]
    pub work_policies: HasMany<super::user_work_policy::Entity>,
    #[sea_orm(has_many, relation_enum = "CreatedWorkPolicies", via_rel = "Creator")]
    pub created_work_policies: HasMany<super::user_work_policy::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

// `find_related` from a user returns the policies that apply to them, not the ones they created.
impl Related<super::user_work_policy::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::WorkPolicies.def()
    }
}
