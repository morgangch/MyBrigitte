use axum::{
    extract::State,
    http::header::SET_COOKIE,
    response::{IntoResponse, Redirect},
};

use crate::{
    AppState,
    auth::{microsoft, refresh::generate_refresh_token},
    error::ApiError,
};

pub async fn login(State(state): State<AppState>) -> Result<impl IntoResponse, ApiError> {
    let config = state.microsoft.as_ref().ok_or(ApiError::NotFound)?;
    let login_state = generate_refresh_token();
    let nonce = generate_refresh_token();
    let url = microsoft::authorize_url(config, &login_state, &nonce);
    Ok((
        [(SET_COOKIE, microsoft::login_cookie(&login_state, &nonce))],
        Redirect::to(url.as_str()),
    ))
}
