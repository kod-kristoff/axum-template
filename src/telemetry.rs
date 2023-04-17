// use opentelemetry::sdk::trace::Tracer;
use opentelemetry::sdk::export::trace::stdout;
use opentelemetry::trace::Tracer;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_opentelemetry::PreSampledTracer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::prelude::*;
use tracing_subscriber::Registry;

pub fn init_telemetry() {
    // let fmt_layer = tracing_subscriber::fmt::layer();

    let tracer = stdout::new_pipeline().install_simple();
    let telemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    // Registry::default()
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "axum_template=debug,tower_http=debug,mio=warn".into()),
        )
        // .with(fmt_layer)
        .with(telemetry_layer)
        .init();
}

fn create_tracer() -> opentelemetry::sdk::trace::Tracer {
    let tracer = opentelemetry::sdk::export::trace::stdout::new_pipeline();
    tracer.install_simple()
    // .install_batch(opentelemetry::runtime::Tokio)
    // .expect("install batch")
}
