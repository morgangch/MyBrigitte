use sea_orm::entity::prelude::{DateTimeWithTimeZone, Uuid};
use serde::{Deserialize, Deserializer, Serialize};

use crate::entity::user;
use crate::models::team::TeamRef;
use crate::services::user::UserWithTeam;

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
    /// `None` only for accounts created outside the API, such as the first seeded manager.
    pub team: Option<TeamRef>,
    pub is_active: bool,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Debug, Deserialize)]
/// Body of `POST /users`. There is no password: the user signs in with Microsoft,
/// then can set one through `/me/password`.
pub struct CreateUserRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_number: Option<String>,
    /// Every user belongs to a team from creation.
    pub team_id: Uuid,
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
    pub team_id: Option<Uuid>,
    pub role: Option<UserRole>,
    /// Case-insensitive search in the full name and the email.
    pub q: Option<String>,
}

#[derive(Debug, Deserialize)]
/// Body of `PATCH /users/{id}`. The password is not editable here: it belongs to the user.
pub struct UpdateUserRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_phone")]
    pub phone_number: Option<Option<String>>,
    pub role: Option<UserRole>,
    /// Moves the user to this team. A user cannot be left without a team.
    pub team_id: Option<Uuid>,
    pub is_active: Option<bool>,
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

impl From<UserWithTeam> for UserResponse {
    fn from(UserWithTeam { user: model, team }: UserWithTeam) -> Self {
        Self {
            id: model.id,
            first_name: model.first_name,
            last_name: model.last_name,
            email: model.email,
            phone_number: model.phone_number,
            role: model.role.into(),
            team: team.map(TeamRef::from),
            is_active: model.is_active,
            created_at: model.created_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_user_without_role_defaults_to_employee() {
        let parsed: CreateUserRequest = serde_json::from_str(
            r#"{"first_name":"Ada","last_name":"Lovelace","email":"ada@x.com","team_id":"00000000-0000-0000-0000-000000000001"}"#,
        )
        .unwrap();
        assert_eq!(parsed.role, UserRole::Employee);
    }

    #[test]
    fn create_user_keeps_an_explicit_manager_role() {
        let parsed: CreateUserRequest = serde_json::from_str(
            r#"{"first_name":"Ada","last_name":"Lovelace","email":"ada@x.com","team_id":"00000000-0000-0000-0000-000000000001","role":"manager"}"#,
        )
        .unwrap();
        assert_eq!(parsed.role, UserRole::Manager);
    }
}
