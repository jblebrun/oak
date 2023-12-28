use crate::{
    orchestrator_client::OrchestratorClient,
    proto::oak::containers::user_silo::{
        t_oak_user_silo_server::{TOakUserSilo, TOakUserSiloServer},
        ApplicationConfig, TInvokeRequest, TInvokeResponse,
    },
};
use anyhow::anyhow;
use log::info;
use oak_crypto::encryptor::AsyncServerEncryptor;
use prost::DecodeError;
use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;

struct TOakUserSiloImplementation {
    orchestrator_client: OrchestratorClient,
}

impl TOakUserSiloImplementation {
    pub fn new(
        orchestrator_client: OrchestratorClient,
        application_config: ApplicationConfig,
    ) -> Result<Self, DecodeError> {
        println!("Application config: {:?}", application_config);
        Ok(Self {
            orchestrator_client,
        })
    }
}

#[tonic::async_trait]
impl TOakUserSilo for TOakUserSiloImplementation {
    async fn t_invoke(
        &self,
        request: tonic::Request<TInvokeRequest>,
    ) -> Result<tonic::Response<TInvokeResponse>, tonic::Status> {
        let encrypted_request =
            request
                .into_inner()
                .encrypted_request
                .ok_or(tonic::Status::invalid_argument(
                    "encrypted request wasn't provided",
                ))?;

        let mut server_encryptor = AsyncServerEncryptor::new(&self.orchestrator_client);

        // Associated data is ignored.
        let (request_bytes, _) =
            server_encryptor
                .decrypt(&encrypted_request)
                .await
                .map_err(|error| {
                    tonic::Status::invalid_argument(format!(
                        "couldn't decrypt request: {:?}",
                        error
                    ))
                })?;

        info!(
            "Got the request: {}",
            String::from_utf8_lossy(&request_bytes)
        );

        Err(tonic::Status::unimplemented("not yet implemented"))
    }
}

pub async fn create(
    listener: TcpListener,
    orchestrator_client: OrchestratorClient,
    application_config: ApplicationConfig,
) -> Result<(), anyhow::Error> {
    tonic::transport::Server::builder()
        .add_service(TOakUserSiloServer::new(TOakUserSiloImplementation::new(
            orchestrator_client,
            application_config,
        )?))
        .serve_with_incoming(TcpListenerStream::new(listener))
        .await
        .map_err(|error| anyhow!("server error: {:?}", error))
}
