use tracing_subscriber::prelude::*;

pub fn init_telemetry() {
    let fmt_layer = tracing_subscriber::fmt::layer();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "axum_template=debug,tower_http=debug,mio=warn".into()),
        )
        .with(fmt_layer)
        .init();
}
