mod common;

use axum::{
    body::Body,
    http::{
        Request, StatusCode,
        header::{LOCATION, SET_COOKIE},
    },
    response::Response,
};
use backend::{AppState, config::MicrosoftConfig, create_app};
use tower::ServiceExt;

fn state_with_microsoft() -> AppState {
    let mut state = AppState::disconnect(common::TEST_SECRET.to_string());
    state.microsoft = Some(MicrosoftConfig {
        client_id: "client-123".to_string(),
        client_secret: "super-secret-value".to_string(),
        tenant_id: "tenant-456".to_string(),
        redirect_uri: "http://localhost:3001/auth/microsoft/callback".to_string(),
    });
    state
}

async fn get_login(state: AppState) -> Response {
    let request = Request::builder()
        .uri("/auth/microsoft/login")
        .body(Body::empty())
        .unwrap();
    create_app()
        .with_state(state)
        .oneshot(request)
        .await
        .unwrap()
}

#[tokio::test]
async fn microsoft_login_redirects_to_the_tenant() {
    let response = get_login(state_with_microsoft()).await;
    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    let location = response.headers()[LOCATION].to_str().unwrap();
    assert!(
        location.starts_with("https://login.microsoftonline.com/tenant-456/oauth2/v2.0/authorize?")
    );
}

#[tokio::test]
async fn microsoft_login_cookie_matches_the_redirect() {
    let response = get_login(state_with_microsoft()).await;
    let location = response.headers()[LOCATION].to_str().unwrap();
    let cookie = response.headers()[SET_COOKIE].to_str().unwrap();
    let value = cookie
        .strip_prefix("microsoft_login=")
        .unwrap()
        .split(';')
        .next()
        .unwrap();
    let (state, nonce) = value.split_once('.').unwrap();
    assert!(location.contains(&format!("state={state}")));
    assert!(location.contains(&format!("nonce={nonce}")));
}

#[tokio::test]
async fn microsoft_login_without_settings_is_not_found() {
    let state = AppState::disconnect(common::TEST_SECRET.to_string());
    let response = get_login(state).await;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
