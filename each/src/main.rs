mod auth;
mod executor;
mod provoker;
mod mesh;

use crate::provoker::rpc::{ProvokerServer, ProvokerService};
use tokio;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let provoker_rpc = ProvokerService::default();

    Server::builder()
        .add_service(ProvokerServer::new(provoker_rpc))
        .serve(addr)
        .await?;

    Ok(())
}
