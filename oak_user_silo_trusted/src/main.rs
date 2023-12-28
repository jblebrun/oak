use anyhow::anyhow;
use log::info;
use oak_user_silo_trusted::{
    orchestrator_client::OrchestratorClient, proto::oak::containers::user_silo::ApplicationConfig,
};
use prost::Message;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::net::TcpListener;

const TRUSTED_APP_PORT: u16 = 8080;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let mut client = OrchestratorClient::create()
        .await
        .map_err(|error| anyhow!("couldn't create Orchestrator client: {:?}", error))?;
    let application_config =
        ApplicationConfig::decode(client.clone().get_application_config().await?.as_slice())?;
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), TRUSTED_APP_PORT);
    let listener = TcpListener::bind(addr).await?;
    let join_handle = tokio::spawn(oak_user_silo_trusted::oak_user_silo::create(
        listener,
        client.clone(),
        application_config,
    ));
    info!("Notifying ready...");
    client.notify_app_ready().await?;
    info!("Awaiting finish...");
    join_handle.await??;
    info!("Finished...");
    Ok(())
}
