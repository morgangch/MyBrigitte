use crate::entity::user::{self, Model, Role};
use crate::error::ApiError;
use sea_orm::entity::prelude::Uuid;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel, Set};
use std::collections::HashMap;

// Delete (Deactivate to not break the KPIs)
pub async fn deactivate(db: &DatabaseConnection, id: Uuid) -> Result<Model, ApiError> {
    let model = find_by_id(db, id).await?;
    let mut active_model = model.into_active_model();

    active_model.is_active = Set(false);
    active_model.updated_at = Set(chrono::Utc::now().fixed_offset());

    active_model.update(db).await.map_err(ApiError::from_db)
}

pub async fn delete(db: &DatabaseConnection, id: Uuid) -> Result<(), ApiError> {
    let result = user::Entity::delete_by_id(id)
        .exec(db)
        .await
        .map_err(ApiError::from_db)?;

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
    let mut fields = HashMap::new();
    if let Some(first_name) = &input.first_name {
        check_name(&mut fields, "first_name", first_name.trim());
    }
    if let Some(last_name) = &input.last_name {
        check_name(&mut fields, "last_name", last_name.trim());
    }
    if let Some(email) = &input.email {
        check_email(&mut fields, email.trim());
    }
    if let Some(phone_number) = &input.phone_number {
        check_phone(&mut fields, phone_number.trim());
    }
    reject_invalid_fields(fields)?;

    let model = find_by_id(db, id).await?;
    let mut active_model = model.into_active_model();

    if let Some(first_name) = input.first_name {
        active_model.first_name = Set(first_name.trim().to_string());
    }
    if let Some(last_name) = input.last_name {
        active_model.last_name = Set(last_name.trim().to_string());
    }
    if let Some(email) = input.email {
        active_model.email = Set(email.trim().to_string());
    }
    if let Some(phone_number) = input.phone_number {
        active_model.phone_number = Set(Some(phone_number.trim().to_string()));
    }
    if let Some(password) = input.password {
        let password_hash =
            crate::auth::password::hash_password(&password).map_err(|_| ApiError::Internal)?;
        active_model.password_hash = Set(Some(password_hash));
    }
    if let Some(role) = input.role {
        active_model.role = Set(role);
    }
    active_model.updated_at = Set(chrono::Utc::now().fixed_offset());

    active_model.update(db).await.map_err(ApiError::from_db)
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
    let first_name = input.first_name.trim();
    let last_name = input.last_name.trim();
    let email = input.email.trim();
    let mut fields: HashMap<String, String> = HashMap::new();
    check_name(&mut fields, "first_name", first_name);
    check_name(&mut fields, "last_name", last_name);
    check_email(&mut fields, email);

    // Check of Phone only if it's here
    if let Some(phone_number) = &input.phone_number {
        check_phone(&mut fields, phone_number.trim());
    }

    reject_invalid_fields(fields)?;
    let phone_number = input.phone_number.map(|phone| phone.trim().to_string());

    let password_hash = match input.password {
        Some(password) => {
            Some(crate::auth::password::hash_password(&password).map_err(|_| ApiError::Internal)?)
        }
        None => None,
    };

    let active_model = user::ActiveModel {
        first_name: Set(first_name.to_string()),
        last_name: Set(last_name.to_string()),
        email: Set(email.to_string()),
        phone_number: Set(phone_number),
        password_hash: Set(password_hash),
        role: Set(input.role),
        is_active: Set(true),
        ..Default::default()
    };

    active_model.insert(db).await.map_err(ApiError::from_db)
}

// Read
pub async fn find_by_id(db: &DatabaseConnection, id: Uuid) -> Result<Model, ApiError> {
    user::Entity::find_by_id(id)
        .one(db)
        .await
        .map_err(ApiError::from_db)?
        .ok_or(ApiError::NotFound)
}

pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<Model>, ApiError> {
    user::Entity::find()
        .all(db)
        .await
        .map_err(ApiError::from_db)
}

// Validation
fn reject_invalid_fields(fields: HashMap<String, String>) -> Result<(), ApiError> {
    if fields.is_empty() {
        Ok(())
    } else {
        Err(ApiError::ValidationFailed(fields))
    }
}

fn email_valid_format(email: &str) -> bool {
    let mut parts = email.split('@');
    match (parts.next(), parts.next(), parts.next()) {
        (Some(local), Some(domain), None) => {
            !local.is_empty()
                && domain.contains('.')
                && !domain.starts_with('.')
                && !domain.ends_with('.')
        }
        _ => false,
    }
}

fn check_name(fields: &mut HashMap<String, String>, key: &str, value: &str) {
    if value.is_empty() {
        fields.insert(key.to_string(), "required".to_string());
    } else if value.chars().count() > 100 {
        fields.insert(key.to_string(), "too long".to_string());
    }
}

/// Check if the email is well formed
fn check_email(fields: &mut HashMap<String, String>, email: &str) {
    if email.is_empty() {
        fields.insert("email".to_string(), "required".to_string());
    } else if email.chars().count() > 254 {
        fields.insert("email".to_string(), "too long".to_string());
    } else if !email_valid_format(email) {
        fields.insert("email".to_string(), "invalid format".to_string());
    }
}

fn check_phone(fields: &mut HashMap<String, String>, phone: &str) {
    if phone.is_empty() {
        fields.insert("phone_number".to_string(), "required".to_string());
    } else if phone.chars().count() > 20 {
        fields.insert("phone_number".to_string(), "too long".to_string());
    }
}
