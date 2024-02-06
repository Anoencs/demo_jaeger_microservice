use tonic::transport::Server;
use app_proto::{otel_jaeger_init, mul::mul_server::MulServer};

mod mul;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    otel_jaeger_init("mul-service")?;

    Server::builder()
        .add_service(MulServer::new(mul::MulService::new().await?))
        .serve("127.0.0.1:12342".parse()?)
        .await?;

    Ok(())
}
