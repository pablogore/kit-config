use std::fs::File;
use std::io::Write;
use tempfile::TempDir;

use kit_config::loader::ConfigLoader;
use kit_config::{ConfigError, LoggingConfig};

fn write_toml(temp_dir: &TempDir, contents: &str) -> String {
    let config_file = temp_dir.path().join("config.toml");
    let mut file = File::create(&config_file).unwrap();
    write!(file, "{contents}").unwrap();
    config_file.to_str().unwrap().to_string()
}

#[test]
fn test_load_and_validate_returns_ok_when_structurally_and_semantically_valid() {
    let temp_dir = TempDir::new().unwrap();
    let path = write_toml(
        &temp_dir,
        r#"
enabled = true
level = "info"
format = "json"
"#,
    );

    let loader = ConfigLoader::builder().add_toml(path).build().unwrap();

    let result = loader.load_and_validate::<LoggingConfig>();

    assert!(result.is_ok());
}

#[test]
fn test_load_and_validate_returns_validation_error_when_invariant_fails() {
    let temp_dir = TempDir::new().unwrap();
    let path = write_toml(
        &temp_dir,
        r#"
enabled = true
level = "info"
format = "json"

[sampling]
enabled = true
strategy = "probabilistic"
rate = 1.5
n = 100
max_events_per_second = 500
"#,
    );

    let loader = ConfigLoader::builder().add_toml(path).build().unwrap();

    let result = loader.load_and_validate::<LoggingConfig>();

    match result {
        Err(ConfigError::Validation(report)) => {
            assert!(!report.is_valid);
            assert!(
                report
                    .domain_errors
                    .iter()
                    .any(|error| error.field == "sampling.rate"),
                "expected a sampling.rate domain error, got {:?}",
                report.domain_errors
            );
        }
        other => panic!("expected Err(ConfigError::Validation(_)), got {other:?}"),
    }
}

#[test]
fn test_load_and_validate_short_circuits_on_structural_failure_before_validate() {
    let temp_dir = TempDir::new().unwrap();
    let path = write_toml(
        &temp_dir,
        r#"
enabled = true
level = "not_a_real_level"
format = "json"
"#,
    );

    let loader = ConfigLoader::builder().add_toml(path).build().unwrap();

    let result = loader.load_and_validate::<LoggingConfig>();

    match result {
        Err(ConfigError::SerializationError(_)) => {}
        other => panic!("expected Err(ConfigError::SerializationError(_)), got {other:?}"),
    }
}
