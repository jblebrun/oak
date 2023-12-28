pub mod proto {
    pub mod oak {
        pub mod containers {
            tonic::include_proto!("oak.containers");
            pub mod user_silo {
                tonic::include_proto!("oak.containers.user_silo");
            }
        }
        pub use oak_crypto::proto::oak::crypto;
        pub use oak_remote_attestation::proto::oak::{attestation, session};
    }
}

pub mod oak_user_silo;
pub mod orchestrator_client;
