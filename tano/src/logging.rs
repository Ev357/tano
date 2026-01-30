use std::env;

use color_eyre::eyre::Result;
use tano_shared::get_data_dir::get_data_dir;
use tracing_appender::{non_blocking::WorkerGuard, rolling};
use tracing_error::ErrorLayer;
use tracing_subscriber::{self, EnvFilter, Layer, layer::SubscriberExt, util::SubscriberInitExt};

pub async fn initialize_logging() -> Result<Option<WorkerGuard>> {
    let env_filter = env::var("RUST_LOG").or_else(|_| env::var("TANO_LOG"));

    let log_filter_str = match env_filter {
        Ok(string) if !string.is_empty() => string,
        _ => return Ok(None),
    };

    let directory = get_data_dir()?;

    let file_appender = rolling::never(directory, "tano.log");

    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let file_subscriber = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_writer(non_blocking)
        .with_target(true)
        .with_ansi(false)
        .with_filter(EnvFilter::builder().parse_lossy(log_filter_str));

    tracing_subscriber::registry()
        .with(file_subscriber)
        .with(ErrorLayer::default())
        .init();

    Ok(Some(guard))
}
