use std::net::SocketAddr;

use axum::Json;

use crate::protocol;

#[tracing::instrument]
pub async fn run(addr: SocketAddr) -> anyhow::Result<()> {
    tracing::info!("Starting.");
    let routes =
        axum::Router::new().route("/echo", axum::routing::post(echo));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("Listening.");
    axum::serve(listener, routes).await?;
    Ok(())
}

async fn echo(Json(msg): Json<protocol::Echo>) -> Json<protocol::Echo> {
    Json(msg)
}
