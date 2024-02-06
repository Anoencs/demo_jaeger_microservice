use add::AddService;
use tonic::transport::Server;
use app_proto::{otel_jaeger_init, add::add_server::AddServer};

mod add;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    otel_jaeger_init("add-service")?;
    Server::builder()
        .add_service(AddServer::new(AddService))
        .serve("127.0.0.1:12341".parse()?)
        .await?;

    Ok(())
}
