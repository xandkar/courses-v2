use std::net::SocketAddr;

use super::Srv;

#[derive(clap::Args, Debug, Clone)]
pub struct Cmd {
    #[clap(default_value = "hello event sourcing")]
    payload: String,
}

impl Cmd {
    pub async fn run(&self, srv: &Srv) -> anyhow::Result<()> {
        tracing::info!(?srv, "Starting server.");
        let payload = self.payload.clone();
        let sock_addr = SocketAddr::from((srv.addr, srv.port));
        let routes = axum::Router::new().route("/", axum::routing::get(|| async { payload }));
        let listener = tokio::net::TcpListener::bind(sock_addr).await?;
        tracing::info!(?sock_addr, "Listening.");
        axum::serve(listener, routes).await?;
        Ok(())
    }
}
