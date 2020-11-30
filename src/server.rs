//! Registrant server module.

use crate::proto::naas::registry_server::Registry as RegistryController;
use crate::proto::naas::registry_server::RegistryServer;
use crate::proto::naas::RegisterRequest;
use crate::proto::naas::RegisterResponse;
use crate::proto::naas::Result as ResponseResult;
use crate::proto::naas::Status as ResponseStatus;

use crate::registry::Registry;

use tonic::transport::Server;
use tonic::Request;
use tonic::Response;
use tonic::Status;

use uuid::Uuid;

#[derive(Default)]
pub struct RegistryControllerImpl {
    registry: crate::registry::RegistryImpl,
}

#[tonic::async_trait]
impl RegistryController for RegistryControllerImpl {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let req: RegisterRequest = request.into_inner();

        self.registry
            .register(
                Uuid::parse_str(req.info.as_ref().unwrap().uuid.as_str()).unwrap(),
                req.info.as_ref().unwrap().hostname.as_str(),
            )
            .await
            .unwrap();

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
pub struct RegistrantServer {}

impl RegistrantServer {
    pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
        let addr = "127.0.0.1:55555".parse().unwrap();
        let registry_impl = RegistryControllerImpl::default();

        Server::builder()
            .add_service(RegistryServer::new(registry_impl))
            .serve(addr)
            .await?;

        Ok(())
    }
}
