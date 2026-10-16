use sea_orm::entity::prelude::{DateTimeWithTimeZone, Uuid};
use serde::{Deserialize, Deserializer, Serialize};

use crate::entity::user::{self, Model};

/// Distinguishes a missing `phone_number` key from JSON `null`.
///
/// A plain `Option<String>` collapses both to `None`. This wraps the parsed value in `Some`,
/// so `null` becomes `Some(None)` and a string becomes `Some(Some(value))`.
/// A missing key never calls this function: `#[serde(default)]` supplies `None`.
fn deserialize_optional_phone<'de, D>(deserializer: D) -> Result<Option<Option<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    let phone = Option::<String>::deserialize(deserializer)?;
    Ok(Some(phone))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    #[default]
    Employee,
    Manager,
}

#[derive(Debug, Clone, Serialize)]
/// User returned by the API. `password_hash` is not part of this struct.
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
    /// Missing key becomes `employee`. JSON `null` is still rejected.
    #[serde(default)]
    pub role: UserRole,
}

#[derive(Debug, Deserialize)]
/// Query string of `GET /users`. Absent fields are filled in by [`crate::handlers::user::list_users`].
pub struct ListUsersQuery {
    pub limit: Option<u64>,
    pub offset: Option<u64>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_phone")]
    pub phone_number: Option<Option<String>>,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_user_without_role_defaults_to_employee() {
        let parsed: CreateUserRequest = serde_json::from_str(
            r#"{"first_name":"Ada","last_name":"Lovelace","email":"ada@x.com"}"#,
        )
        .unwrap();
        assert_eq!(parsed.role, UserRole::Employee);
    }

    #[test]
    fn create_user_keeps_an_explicit_manager_role() {
        let parsed: CreateUserRequest = serde_json::from_str(
            r#"{"first_name":"Ada","last_name":"Lovelace","email":"ada@x.com","role":"manager"}"#,
        )
        .unwrap();
        assert_eq!(parsed.role, UserRole::Manager);
    }
}
