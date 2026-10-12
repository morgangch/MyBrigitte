use backend::config::Config;
use backend::create_app;

#[tokio::main]
async fn main() {
    let config = Config::from_env().expect("invalid configuration");
    let app = create_app();

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", config.port))
        .await
        .expect("failed to bind tcp listener");

    println!("Server running on http://localhost:{}", config.port);

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
