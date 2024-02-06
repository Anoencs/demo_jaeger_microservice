use app_proto::{otel_jaeger_init, maths::maths_server::MathsServer};
use tonic::transport::Server;

mod gateway;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    otel_jaeger_init("gateway")?;

    let maths_service = MathsServer::new(gateway::MathsService::new().await?);

    Server::builder()
        .add_service(maths_service)
        .serve("127.0.0.1:12340".parse()?)
        .await?;

    opentelemetry::global::shutdown_tracer_provider();
    Ok(())
}
