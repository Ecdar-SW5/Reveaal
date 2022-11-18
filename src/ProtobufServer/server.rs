use crate::ProtobufServer::services::ecdar_backend_server::EcdarBackendServer;
use crate::ProtobufServer::ConcreteEcdarBackend;
use core::time::Duration;
use log::info;
use tokio::runtime;
use tonic::transport::Server;

pub fn start_grpc_server_with_tokio(ip_endpoint: &str) -> Result<(), Box<dyn std::error::Error>> {
    //For information on switching to a multithreaded server see:
    //https://docs.rs/tokio/1.12.0/tokio/runtime/index.html#multi-thread-scheduler
    let single_threaded_runtime = runtime::Builder::new_current_thread()
        .enable_time()
        .enable_io()
        .build()?;

    single_threaded_runtime.block_on(async { start_grpc_server(ip_endpoint).await })
}

async fn start_grpc_server(ip_endpoint: &str) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting grpc server on '{}'", ip_endpoint.trim());

    Server::builder()
        .http2_keepalive_interval(Some(Duration::from_secs(120)))
        .add_service(EcdarBackendServer::new(ConcreteEcdarBackend::default()))
        .serve(ip_endpoint.trim().parse()?)
        .await?;

    Ok(())
}
