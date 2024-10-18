use super::Srv;

#[derive(clap::Args, Debug, Clone)]
pub struct Cmd {}

impl Cmd {
    #[tracing::instrument(name = "get", skip_all)]
    pub async fn run(&self, srv: &Srv) -> anyhow::Result<()> {
        tracing::info!(?srv, "Starting server.");
        let url = format!("http://{}:{}/", srv.addr, srv.port);
        let body = reqwest::get(&url).await?.text().await?;
        println!("{body}");
        Ok(())
    }
}
