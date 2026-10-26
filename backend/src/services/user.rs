use crate::entity::team;
use crate::entity::team_member;
use crate::entity::user::{self, Model, Role};
use crate::error::ApiError;
use sea_orm::entity::prelude::Uuid;
use sea_orm::sea_query::{Expr, Query};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, ConnectionTrait, DatabaseConnection, EntityTrait,
    IntoActiveModel, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Set, TransactionTrait,
};
use std::collections::HashMap;

/// A user and their team, the shape the API returns.
pub struct UserWithTeam {
    pub user: Model,
    /// `None` only for accounts created outside the API, such as the first seeded manager.
    pub team: Option<team::Model>,
}

/// Sets `is_active` to false and keeps the row.
///
/// Clock entries and other tables reference the user, so a SQL `DELETE` would fail or drop history.
/// HTTP `DELETE /users/{id}` calls this function.
pub async fn deactivate(db: &DatabaseConnection, id: Uuid) -> Result<Model, ApiError> {
    let model = find_model(db, id).await?;
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
    pub role: Option<Role>,
    /// Moves the user to this team.
    pub team_id: Option<Uuid>,
    pub is_active: Option<bool>,
}

/// Applies the fields the client sent. Omitted fields are not written.
///
/// Field checks run before the row is loaded, and the stored text is the trimmed text.
/// The row and the team move are written in one transaction: an unknown team leaves the user unchanged.
pub async fn update(
    db: &DatabaseConnection,
    id: Uuid,
    input: UpdateUser,
) -> Result<UserWithTeam, ApiError> {
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

    let txn = db.begin().await.map_err(ApiError::from_db)?;
    let model = find_model(&txn, id).await?;
    let team = match input.team_id {
        Some(team_id) => Some(find_team(&txn, team_id).await?),
        None => None,
    };
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
    if let Some(role) = input.role {
        active_model.role = Set(role);
    }
    if let Some(is_active) = input.is_active {
        active_model.is_active = Set(is_active);
    }
    active_model.updated_at = Set(chrono::Utc::now().fixed_offset());

    let updated = active_model.update(&txn).await.map_err(ApiError::from_db)?;
    if let Some(team) = &team {
        move_to_team(&txn, id, team.id).await?;
    }
    txn.commit().await.map_err(ApiError::from_db)?;

    let team = match team {
        Some(team) => Some(team),
        None => teams_by_user(db, &[id]).await?.remove(&id),
    };
    Ok(UserWithTeam {
        user: updated,
        team,
    })
}

/// Values for an insert. `phone_number: None` is stored as SQL `NULL`.
pub struct CreateUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub team_id: Uuid,
    pub role: Role,
}

/// Inserts an active user without a password, and their team membership, in one transaction.
///
/// Names, email, and phone are checked before SQL. A blank email is then
/// `validation_failed` (400) instead of the `users_email_not_blank` check failing as 500.
/// An unknown team is `validation_failed` on `team_id`. Stored strings are the trimmed ones.
pub async fn create(db: &DatabaseConnection, input: CreateUser) -> Result<UserWithTeam, ApiError> {
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

    let txn = db.begin().await.map_err(ApiError::from_db)?;
    let team = find_team(&txn, input.team_id).await?;

    let created = user::ActiveModel {
        first_name: Set(first_name.to_string()),
        last_name: Set(last_name.to_string()),
        email: Set(email.to_string()),
        phone_number: Set(phone_number),
        password_hash: Set(None),
        role: Set(input.role),
        is_active: Set(true),
        ..Default::default()
    }
    .insert(&txn)
    .await
    .map_err(ApiError::from_db)?;

    move_to_team(&txn, created.id, team.id).await?;
    txn.commit().await.map_err(ApiError::from_db)?;

    Ok(UserWithTeam {
        user: created,
        team: Some(team),
    })
}

pub async fn find_by_id(db: &DatabaseConnection, id: Uuid) -> Result<UserWithTeam, ApiError> {
    let user = find_model(db, id).await?;
    let team = teams_by_user(db, &[id]).await?.remove(&id);
    Ok(UserWithTeam { user, team })
}

async fn find_model<C: ConnectionTrait>(db: &C, id: Uuid) -> Result<Model, ApiError> {
    user::Entity::find_by_id(id)
        .one(db)
        .await
        .map_err(ApiError::from_db)?
        .ok_or(ApiError::NotFound)
}

/// A team the client named in a body. Missing is a `team_id` field error, not a 404 on the route.
async fn find_team<C: ConnectionTrait>(db: &C, id: Uuid) -> Result<team::Model, ApiError> {
    team::Entity::find_by_id(id)
        .one(db)
        .await
        .map_err(ApiError::from_db)?
        .ok_or_else(|| {
            ApiError::ValidationFailed(HashMap::from([(
                "team_id".to_string(),
                "not found".to_string(),
            )]))
        })
}

/// Puts the user in `team_id`, leaving their previous team if any.
/// `joined_at` restarts only when the team actually changes.
async fn move_to_team<C: ConnectionTrait>(
    db: &C,
    user_id: Uuid,
    team_id: Uuid,
) -> Result<(), ApiError> {
    let current = team_member::Entity::find_by_id(user_id)
        .one(db)
        .await
        .map_err(ApiError::from_db)?;

    match current {
        Some(member) if member.team_id == team_id => {}
        Some(member) => {
            let mut active_model = member.into_active_model();
            active_model.team_id = Set(team_id);
            active_model.joined_at = Set(chrono::Utc::now().fixed_offset());
            active_model.update(db).await.map_err(ApiError::from_db)?;
        }
        None => {
            team_member::ActiveModel {
                user_id: Set(user_id),
                team_id: Set(team_id),
                ..Default::default()
            }
            .insert(db)
            .await
            .map_err(ApiError::from_db)?;
        }
    }

    Ok(())
}

/// The team of each listed user who has one, in two queries whatever the number of users.
async fn teams_by_user<C: ConnectionTrait>(
    db: &C,
    user_ids: &[Uuid],
) -> Result<HashMap<Uuid, team::Model>, ApiError> {
    let members = team_member::Entity::find()
        .filter(team_member::Column::UserId.is_in(user_ids.iter().copied()))
        .all(db)
        .await
        .map_err(ApiError::from_db)?;

    let teams: HashMap<Uuid, team::Model> = team::Entity::find()
        .filter(team::Column::Id.is_in(members.iter().map(|member| member.team_id)))
        .all(db)
        .await
        .map_err(ApiError::from_db)?
        .into_iter()
        .map(|team| (team.id, team))
        .collect();

    Ok(members
        .into_iter()
        .filter_map(|member| {
            teams
                .get(&member.team_id)
                .map(|team| (member.user_id, team.clone()))
        })
        .collect())
}

/// Filters of `GET /users`. `None` means "do not filter on this".
pub struct UserFilter {
    pub is_active: bool,
    pub team_id: Option<Uuid>,
    pub role: Option<Role>,
    /// Matched case-insensitively against "first_name last_name" and the email.
    pub q: Option<String>,
}

pub struct ListedUsers {
    pub items: Vec<UserWithTeam>,
    /// How many rows match the filter, including those outside this page.
    pub total: u64,
}

/// One page of users matching `filter`, ordered by last name, first name, then id.
///
/// The sort keeps page 2 the continuation of page 1. `limit` and `offset` are applied only to `items`.
pub async fn find_page(
    db: &DatabaseConnection,
    limit: u64,
    offset: u64,
    filter: UserFilter,
) -> Result<ListedUsers, ApiError> {
    let mut query = user::Entity::find().filter(user::Column::IsActive.eq(filter.is_active));

    if let Some(team_id) = filter.team_id {
        query = query.filter(
            user::Column::Id.in_subquery(
                Query::select()
                    .column(team_member::Column::UserId)
                    .from(team_member::Entity)
                    .and_where(team_member::Column::TeamId.eq(team_id))
                    .to_owned(),
            ),
        );
    }
    if let Some(role) = filter.role {
        query = query.filter(user::Column::Role.eq(role));
    }
    if let Some(q) = filter.q.as_deref().map(str::trim).filter(|q| !q.is_empty()) {
        let pattern = format!("%{}%", escape_like(q));
        query = query.filter(
            Condition::any()
                .add(Expr::cust_with_values(
                    "(first_name || ' ' || last_name) ILIKE $1",
                    [pattern.clone()],
                ))
                .add(Expr::cust_with_values("email ILIKE $1", [pattern])),
        );
    }

    let query = query
        .order_by_asc(user::Column::LastName)
        .order_by_asc(user::Column::FirstName)
        .order_by_asc(user::Column::Id);

    let total = query.clone().count(db).await.map_err(ApiError::from_db)?;
    let users = query
        .offset(offset)
        .limit(limit)
        .all(db)
        .await
        .map_err(ApiError::from_db)?;

    let ids: Vec<Uuid> = users.iter().map(|user| user.id).collect();
    let mut teams = teams_by_user(db, &ids).await?;
    let items = users
        .into_iter()
        .map(|user| {
            let team = teams.remove(&user.id);
            UserWithTeam { user, team }
        })
        .collect();

    Ok(ListedUsers { items, total })
}

/// Makes `%`, `_` and `\` literal in an `ILIKE` pattern, so a search for "50%" is not a wildcard.
fn escape_like(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('%', "\\%")
        .replace('_', "\\_")
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape_like_makes_wildcards_literal() {
        assert_eq!(escape_like("50%_a\\b"), "50\\%\\_a\\\\b");
    }

    #[test]
    fn escape_like_keeps_plain_text() {
        assert_eq!(escape_like("Ada Lovelace"), "Ada Lovelace");
    }
}
