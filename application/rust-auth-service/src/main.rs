use std::net::SocketAddr;

use clap::Parser;
use rust_auth_service::{client, server};
use tracing::{error_span, Instrument};

#[derive(Parser, Debug)]
struct Cli {
    #[clap(short, long, default_value = "127.0.0.1")]
    addr: std::net::IpAddr,

    #[clap(short, long, default_value = "3000")]
    port: u16,

    #[clap(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, clap::Subcommand)]
enum Cmd {
    Serve,
    Echo { text: String },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    tracing_init(tracing::Level::DEBUG)?;
    tracing::debug!(?cli, "Starting.");
    let addr = SocketAddr::from((cli.addr, cli.port));
    let server_span = error_span!("server", ?addr);
    let client_span = error_span!("client", ?addr);
    match cli.cmd {
        Cmd::Serve => {
            server::run(addr).instrument(server_span).await?;
        }
        Cmd::Echo { text } => {
            client::echo(addr, &text).instrument(client_span).await?;
        }
    }
    Ok(())
}

fn tracing_init(level: tracing::Level) -> anyhow::Result<()> {
    use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter, Layer};

    let layer_stderr = fmt::Layer::new()
        .with_writer(std::io::stderr)
        .with_ansi(true)
        .with_file(false)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_filter(
            EnvFilter::from_default_env().add_directive(level.into()),
        );
    tracing::subscriber::set_global_default(
        tracing_subscriber::registry().with(layer_stderr),
    )?;
    Ok(())
}
