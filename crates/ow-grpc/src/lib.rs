pub mod proto {
    tonic::include_proto!("overlayward.v1");
}

mod services;

use ow_core::ServiceRegistry;
use std::{net::SocketAddr, sync::Arc};

pub struct GrpcServer {
    pub registry: Arc<ServiceRegistry>,
    pub port: u16,
}

impl GrpcServer {
    pub fn new(registry: Arc<ServiceRegistry>) -> Self {
        Self { registry, port: 8421 }
    }

    pub fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let addr = SocketAddr::from(([0, 0, 0, 0], self.port));
        tracing::info!("gRPC API listening on {addr}");

        let r = self.registry;
        tonic::transport::Server::builder()
            .add_service(proto::sandbox_service_server::SandboxServiceServer::new(services::SandboxSvc(r.clone())))
            .add_service(proto::snapshot_service_server::SnapshotServiceServer::new(services::SnapshotSvc(r.clone())))
            .add_service(proto::network_service_server::NetworkServiceServer::new(services::NetworkSvc(r.clone())))
            .add_service(proto::exec_service_server::ExecServiceServer::new(services::ExecSvc(r.clone())))
            .add_service(proto::file_service_server::FileServiceServer::new(services::FileSvc(r.clone())))
            .add_service(proto::volume_service_server::VolumeServiceServer::new(services::VolumeSvc(r.clone())))
            .add_service(proto::audit_service_server::AuditServiceServer::new(services::AuditSvc(r.clone())))
            .add_service(proto::resource_service_server::ResourceServiceServer::new(services::ResourceSvc(r.clone())))
            .add_service(proto::inter_service_server::InterServiceServer::new(services::InterSvc(r.clone())))
            .add_service(proto::approval_service_server::ApprovalServiceServer::new(services::ApprovalSvc(r.clone())))
            .add_service(proto::event_service_server::EventServiceServer::new(services::EventSvc(r)))
            .serve(addr)
            .await?;
        Ok(())
    }
}
