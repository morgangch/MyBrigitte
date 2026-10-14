use axum::Router;
use backend::config::Config;
use backend::{AppState, create_app};
use sea_orm::{Database, DatabaseConnection};

#[tokio::main]
async fn main() {
    let config: Config = Config::from_env().expect("invalid configuration");
    let db: DatabaseConnection = Database::connect(&config.database_url)
        .await
        .expect("failed to connect to database");

    let app: Router = create_app().with_state(AppState::new(db));

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", config.port))
        .await
        .expect("failed to bind tcp listener");

    println!("Server running on http://localhost:{}", config.port);

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
