//! Registrant server module.

use crate::proto::naas::RegisterRequest;
use crate::proto::naas::RegisterResponse;
use crate::proto::naas::registry_server::Registry;
use crate::proto::naas::registry_server::RegistryServer;
use crate::proto::naas::Result as ResponseResult;
use crate::proto::naas::Status as ResponseStatus;

use tonic::Request;
use tonic::Response;
use tonic::Status;
use tonic::transport::Server;

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
            status: Some(ResponseStatus {
                result: ResponseResult::Ok.into(),
                err: None,
            }),
        };
        Ok(Response::new(reply))
    }
}

// Container for the registrant RPC server.
#[derive(Default)]
pub struct RegistrantServer {}

impl RegistrantServer {
    pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
        let addr = "127.0.0.1:55555".parse().unwrap();  
        let registry_impl = RegistryImpl::default();

        Server::builder()
            .add_service(RegistryServer::new(registry_impl))
            .serve(addr)
            .await?;
        
        Ok(())
    }
}