use crate::entity::user::{self, Model, Role};
use crate::error::ApiError;
use sea_orm::entity::prelude::Uuid;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel,
    PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Set,
};
use std::collections::HashMap;

/// Sets `is_active` to false and keeps the row.
///
/// Clock entries and other tables reference the user, so a SQL `DELETE` would fail or drop history.
/// HTTP `DELETE /users/{id}` calls this function.
pub async fn deactivate(db: &DatabaseConnection, id: Uuid) -> Result<Model, ApiError> {
    let model = find_by_id(db, id).await?;
    let mut active_model = model.into_active_model();

    active_model.is_active = Set(false);
    active_model.updated_at = Set(chrono::Utc::now().fixed_offset());

    active_model.update(db).await.map_err(ApiError::from_db)
}

/// Removes the row with SQL `DELETE`. The HTTP route does not call this; it calls [`deactivate`].
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

/// Partial update. `None` on a field means the client omitted it, so the column stays as it is.
///
/// `phone_number` has a third state because JSON `null` must clear the column:
/// `None` leaves it unchanged, `Some(None)` writes SQL `NULL`, `Some(Some(value))` replaces it.
pub struct UpdateUser {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    /// See the struct comment for the three states.
    pub phone_number: Option<Option<String>>,
    pub password: Option<String>,
    pub role: Option<Role>,
}

/// Applies the fields the client sent. Omitted fields are not written.
///
/// Checks run before the row is loaded, and the stored text is the trimmed text.
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
    if let Some(Some(phone_number)) = &input.phone_number {
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
    match input.phone_number {
        Some(Some(phone_number)) => {
            active_model.phone_number = Set(Some(phone_number.trim().to_string()));
        }
        Some(None) => {
            active_model.phone_number = Set(None);
        }
        None => {}
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

/// Values for an insert. `phone_number: None` is stored as SQL `NULL`.
pub struct CreateUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub password: Option<String>,
    pub role: Role,
}

/// Inserts an active user.
///
/// Names, email, and phone are checked before hashing and SQL. A blank email is then
/// `validation_failed` (400) instead of the `users_email_not_blank` check failing as 500.
/// Stored strings are the trimmed ones.
pub async fn create(db: &DatabaseConnection, input: CreateUser) -> Result<Model, ApiError> {
    let first_name = input.first_name.trim();
    let last_name = input.last_name.trim();
    let email = input.email.trim();
    let mut fields: HashMap<String, String> = HashMap::new();
    check_name(&mut fields, "first_name", first_name);
    check_name(&mut fields, "last_name", last_name);
    check_email(&mut fields, email);

    // An omitted phone is NULL, not an invalid value.
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

pub async fn find_by_id(db: &DatabaseConnection, id: Uuid) -> Result<Model, ApiError> {
    user::Entity::find_by_id(id)
        .one(db)
        .await
        .map_err(ApiError::from_db)?
        .ok_or(ApiError::NotFound)
}

pub struct ListedUsers {
    pub items: Vec<Model>,
    /// How many rows match the filter, including those outside this page.
    pub total: u64,
}

/// One page of users with the same `is_active` value, ordered by last name, first name, then id.
///
/// The sort keeps page 2 the continuation of page 1. `limit` and `offset` are applied only to `items`.
pub async fn find_page(
    db: &DatabaseConnection,
    limit: u64,
    offset: u64,
    is_active: bool,
) -> Result<ListedUsers, ApiError> {
    let query = user::Entity::find()
        .filter(user::Column::IsActive.eq(is_active))
        .order_by_asc(user::Column::LastName)
        .order_by_asc(user::Column::FirstName)
        .order_by_asc(user::Column::Id);

    let total = query.clone().count(db).await.map_err(ApiError::from_db)?;
    let items = query
        .offset(offset)
        .limit(limit)
        .all(db)
        .await
        .map_err(ApiError::from_db)?;

    Ok(ListedUsers { items, total })
}

/// Turns a non-empty field map into `validation_failed`. An empty map means the input passed.
fn reject_invalid_fields(fields: HashMap<String, String>) -> Result<(), ApiError> {
    if fields.is_empty() {
        Ok(())
    } else {
        Err(ApiError::ValidationFailed(fields))
    }
}

/// One `@`, a non-empty local part, and a domain that contains a dot but does not start or end with one.
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

/// Records `required` or `too long`. `value` must already be trimmed. Names are limited to 100 characters.
fn check_name(fields: &mut HashMap<String, String>, key: &str, value: &str) {
    if value.is_empty() {
        fields.insert(key.to_string(), "required".to_string());
    } else if value.chars().count() > 100 {
        fields.insert(key.to_string(), "too long".to_string());
    }
}

/// Records `required`, `too long`, or `invalid format`. `email` must already be trimmed. The column allows 254 characters.
fn check_email(fields: &mut HashMap<String, String>, email: &str) {
    if email.is_empty() {
        fields.insert("email".to_string(), "required".to_string());
    } else if email.chars().count() > 254 {
        fields.insert("email".to_string(), "too long".to_string());
    } else if !email_valid_format(email) {
        fields.insert("email".to_string(), "invalid format".to_string());
    }
}

/// Records `required` or `too long`. `phone` must already be trimmed. The column allows 20 characters.
///
/// Do not call this for a missing phone: absence is SQL `NULL`, not an error.
fn check_phone(fields: &mut HashMap<String, String>, phone: &str) {
    if phone.is_empty() {
        fields.insert("phone_number".to_string(), "required".to_string());
    } else if phone.chars().count() > 20 {
        fields.insert("phone_number".to_string(), "too long".to_string());
    }
}
