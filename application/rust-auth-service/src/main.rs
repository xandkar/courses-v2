use clap::Parser;
use tracing::{error_span, Instrument};

#[derive(Parser, Debug)]
struct Cli {
    #[clap(flatten)]
    srv: rust_auth_service::cmd::Srv,

    #[clap(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, clap::Subcommand)]
enum Cmd {
    Serve(rust_auth_service::cmd::serve::Cmd),
    Get(rust_auth_service::cmd::get::Cmd),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    tracing_init(tracing::Level::DEBUG)?;
    tracing::debug!(?cli, "Starting.");
    let Cli { srv, cmd } = cli;
    let server_span = error_span!("server", ?srv);
    let client_span = error_span!("client", ?srv);
    match cmd {
        Cmd::Serve(cmd) => {
            cmd.run(&srv).instrument(server_span).await?;
        }
        Cmd::Get(cmd) => {
            cmd.run(&srv).instrument(client_span).await?;
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
        .with_filter(EnvFilter::from_default_env().add_directive(level.into()));
    tracing::subscriber::set_global_default(tracing_subscriber::registry().with(layer_stderr))?;
    Ok(())
}
