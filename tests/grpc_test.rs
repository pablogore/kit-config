use kit_config::modules::infra::grpc::{GrpcModule, GrpcClientModule};
use kit_config::validation::Validation;

#[test]
fn test_grpc_module_defaults() {
    let grpc_config = GrpcModule::defaults();
    assert_eq!(grpc_config.host, "0.0.0.0");
    assert_eq!(grpc_config.port, 50051);
    assert_eq!(grpc_config.tls_enabled, false);
    assert_eq!(grpc_config.tls_cert_path, None);
    assert_eq!(grpc_config.tls_key_path, None);
    assert_eq!(grpc_config.keepalive_time, None);
    assert_eq!(grpc_config.keepalive_timeout, None);
}

#[test]
fn test_grpc_client_module_defaults() {
    let grpc_client_config = GrpcClientModule::defaults();
    assert_eq!(grpc_client_config.target, "localhost:50051");
    assert_eq!(grpc_client_config.tls_enabled, false);
    assert_eq!(grpc_client_config.tls_cert_path, None);
    assert_eq!(grpc_client_config.keepalive_time, None);
    assert_eq!(grpc_client_config.keepalive_timeout, None);
}

#[test]
fn test_grpc_module_validation() {
    let grpc_config = GrpcModule::defaults();
    let result = grpc_config.validate();
    assert!(result.is_ok());
}

#[test]
fn test_grpc_client_module_validation() {
    let grpc_client_config = GrpcClientModule::defaults();
    let result = grpc_client_config.validate();
    assert!(result.is_ok());
}