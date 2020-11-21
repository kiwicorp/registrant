use tonic::{transport::Server, Request, Response, Status};

use registry::registry_server::{Registry, RegistryServer};
use registry::{RegisterRequest, RegisterResponse};

use sandhog::sandhog_client::SandhogClient;
use sandhog::StartTunnelRequest;

pub mod registry {
    tonic::include_proto!("selftechio.registry");
}

pub mod sandhog {
    tonic::include_proto!("selftechio.sandhog");
}

#[derive(Default)]
pub struct RegistryImpl {}

#[tonic::async_trait]
impl Registry for RegistryImpl {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let reply = RegisterResponse {
            message: format!("Hello {}!", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let addr = "127.0.0.1:55555".parse().unwrap();
    // let registry = RegistryImpl::default();

    // println!("RegistryServer listening on {}", addr);

    // Server::builder()
    //     .add_service(RegistryServer::new(registry))
    //     .serve(addr)
    //     .await?;

    // Ok(())
    let mut client = SandhogClient::connect("http://127.0.0.1:22222").await?;

    let request = tonic::Request::new(StartTunnelRequest {
        address: "10.11.12.13/32".into(),
        name: "wgtest0".into(),
    });

    let response = client.start_tunnel(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}