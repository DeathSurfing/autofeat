use autofeat::app;
use autofeat::cli::Cli;
use clap::Parser;

#[tokio::main]
async fn main() -> app::Result<()> {
    let cli = Cli::parse();

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    app::run(cli).await
}
