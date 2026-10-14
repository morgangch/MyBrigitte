use crate::models::user::User;
use sea_orm::entity::prelude::Uuid;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

use crate::entity::user::{self, Model, Role};
use crate::error::ApiError;
const MAX_USER_ID: u32 = 100;

// Create
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub password: Option<String>,
    pub role: Role,
}

pub async fn create(db: &DatabaseConnection, input: NewUser) -> Result<Model, ApiError> {
    let password_hash = match input.password {
        Some(password) => Some(
            crate::auth::password::hash_password(&password).map_err(|_| ApiError::InternalError)?,
        ),
        None => None,
    };

    let active_model = user::ActiveModel {
        first_name: Set(input.first_name),
        last_name: Set(input.last_name),
        email: Set(input.email),
        phone_number: Set(input.phone_number),
        password_hash: Set(password_hash),
        role: Set(input.role),
        is_active: Set(true),
        ..Default::default()
    };

    active_model
        .insert(db)
        .await
        .map_err(|_| ApiError::InternalError)
}

// Read
pub async fn find_by_id(db: &DatabaseConnection, id: Uuid) -> Result<Model, ApiError> {
    user::Entity::find_by_id(id)
        .one(db)
        .await
        .map_err(|_| ApiError::InternalError)?
        .ok_or(ApiError::NotFound)
}

pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<Model>, ApiError> {
    user::Entity::find()
        .all(db)
        .await
        .map_err(|_| ApiError::InternalError)
}

// Demo functions
pub fn list_users() -> Result<Vec<User>, ApiError> {
    // No data source yet
    Err(ApiError::InternalError)
}

pub fn get_user(id: u32) -> Result<User, ApiError> {
    if id > MAX_USER_ID {
        return Err(ApiError::NotFound);
    }

    Ok(User {
        id,
        name: "User".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_user_returns_user_when_id_is_valid() {
        let user = get_user(MAX_USER_ID).unwrap();
        assert_eq!(user.id, MAX_USER_ID);
    }

    #[test]
    fn test_get_user_returns_not_found_when_id_is_too_high() {
        let result = get_user(MAX_USER_ID + 1);
        assert!(matches!(result, Err(ApiError::NotFound)));
    }
}
