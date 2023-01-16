use tokio;
use tonic::transport::Server;

use db_manager::services;
use db_manager::services::web_server::web_server_server::WebServerServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50055".parse().unwrap();
    let service = services::WebServerRPC::default();

    let mut server = Server::builder();

    server
        .add_service(WebServerServer::new(service))
        .serve(addr)
        .await?;
    Ok(())
}
