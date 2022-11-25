use tokio;
use tonic::transport::Server;

use db_manager::services;
use db_manager::services::scraper_rpc::scraper_service_server::ScraperServiceServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let service = services::ScraperRPCService::default();

    let mut server = Server::builder();

    server
        .add_service(ScraperServiceServer::new(service))
        .serve(addr)
        .await?;
    Ok(())
}
