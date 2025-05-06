use axum::Router;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let settings = configs::load()?;

    let state = states::AppState::new(&settings).await?;

    let app = Router::new()
        .nest("/users", handlers::user_handler::router())
        .with_state(state);

    let addr = settings.server_address;
    tracing::info!("Listening on{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
