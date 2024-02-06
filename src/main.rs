use opentelemetry::sdk::trace::Tracer;
use opentelemetry::trace::TraceError;
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::prelude::*;


fn init_tracer() -> Result<Tracer, TraceError> {
    opentelemetry_jaeger::new_pipeline()
        .with_service_name("jaeger_example")
        .install_simple()
}


fn main() -> Result<(), Box<dyn Send + Sync + 'static>> {
    let tracer = init_tracer().expect("Failed to initialize tracer"); //calling our new init_tracer function

    tracing_subscriber::registry() //(1)
        .with(tracing_subscriber::EnvFilter::new("TRACE")) //(2)
        .with(tracing_opentelemetry::layer().with_tracer(tracer)) //(3)
        .try_init()
        .expect("Failed to register tracer with registry");
    opentelemetry::global::shutdown_tracer_provider(); //add this line

    Ok(())
}
