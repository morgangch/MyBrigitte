use crate::{error::ApiError, models::user::User};

const MAX_USER_ID: u32 = 100;

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
