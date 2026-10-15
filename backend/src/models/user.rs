use sea_orm::entity::prelude::{DateTimeWithTimeZone, Uuid};
use serde::{Deserialize, Serialize};

use crate::entity::user::{self, Model};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    Employee,
    Manager,
}

#[derive(Debug, Clone, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub role: UserRole,
    pub is_active: bool,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub password: Option<String>,
    pub role: UserRole,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub password: Option<String>,
    pub role: Option<UserRole>,
}

impl From<user::Role> for UserRole {
    fn from(role: user::Role) -> Self {
        match role {
            user::Role::Employee => Self::Employee,
            user::Role::Manager => Self::Manager,
        }
    }
}

impl From<UserRole> for user::Role {
    fn from(role: UserRole) -> Self {
        match role {
            UserRole::Employee => Self::Employee,
            UserRole::Manager => Self::Manager,
        }
    }
}

impl From<Model> for UserResponse {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            first_name: model.first_name,
            last_name: model.last_name,
            email: model.email,
            phone_number: model.phone_number,
            role: model.role.into(),
            is_active: model.is_active,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
