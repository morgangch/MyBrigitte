use crate::models::user::User;
use sea_orm::entity::prelude::Uuid;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel, Set};

use crate::entity::user::{self, Model, Role};
use crate::error::ApiError;

// Delete (Deactivate to not break the KPIs)
pub async fn deactivate(db: &DatabaseConnection, id: Uuid) -> Result<Model, ApiError> {
    let model = find_by_id(db, id).await?;
    let mut active_model = model.into_active_model();

    active_model.is_active = Set(false);
    active_model.updated_at = Set(chrono::Utc::now().fixed_offset());

    active_model
        .update(db)
        .await
        .map_err(|_| ApiError::InternalError)
}

pub async fn delete(db: &DatabaseConnection, id: Uuid) -> Result<(), ApiError> {
    let result = user::Entity::delete_by_id(id)
        .exec(db)
        .await
        .map_err(|_| ApiError::NotFound)?;

    if result.rows_affected == 0 {
        return Err(ApiError::NotFound);
    }

    Ok(())
}

pub struct UpdateUser {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub password: Option<String>,
    pub role: Option<Role>,
}

pub async fn update(
    db: &DatabaseConnection,
    id: Uuid,
    input: UpdateUser,
) -> Result<Model, ApiError> {
    let model = find_by_id(db, id).await?;
    let mut active_model = model.into_active_model();

    if let Some(first_name) = input.first_name {
        active_model.first_name = Set(first_name);
    }
    if let Some(last_name) = input.last_name {
        active_model.last_name = Set(last_name);
    }
    if let Some(email) = input.email {
        active_model.email = Set(email);
    }
    if let Some(phone_number) = input.phone_number {
        active_model.phone_number = Set(Some(phone_number));
    }
    if let Some(password) = input.password {
        let password_hash =
            crate::auth::password::hash_password(&password).map_err(|_| ApiError::InternalError)?;
        active_model.password_hash = Set(Some(password_hash));
    }
    if let Some(role) = input.role {
        active_model.role = Set(role);
    }
    active_model.updated_at = Set(chrono::Utc::now().fixed_offset());

    active_model
        .update(db)
        .await
        .map_err(|_| ApiError::InternalError)
}

// Create
pub struct CreateUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub password: Option<String>,
    pub role: Role,
}

pub async fn create(db: &DatabaseConnection, input: CreateUser) -> Result<Model, ApiError> {
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
        .map_err(|error| match error.sql_err() {
            Some(sea_orm::SqlErr::UniqueConstraintViolation(_)) => ApiError::Conflict,
            _ => ApiError::InternalError,
        })
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

const MAX_USER_ID: u32 = 100;

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
