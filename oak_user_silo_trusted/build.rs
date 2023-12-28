use oak_grpc_utils::{generate_grpc_code, CodegenOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    generate_grpc_code(
        "..",
        &[
            "oak_user_silo_trusted/proto/t_oak_user_silo.proto",
            "oak_user_silo_trusted/proto/application_config.proto",
            "oak_crypto/proto/v1/crypto.proto",
        ],
        CodegenOptions {
            build_server: true,
            ..Default::default()
        },
    )?;

    generate_grpc_code(
        "..",
        &[
            "oak_containers/proto/interfaces.proto",
            "oak_remote_attestation/proto/v1/messages.proto",
        ],
        CodegenOptions {
            build_client: true,
            ..Default::default()
        },
    )?;
    Ok(())
}
