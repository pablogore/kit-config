use std::fs::File;
use std::io::Write;
use tempfile::TempDir;

#[cfg(test)]
mod tests {
    use super::*;
    use kit_config::sources::TomlFileSource;
    use kit_config::sources::ConfigurationSource;
    use serde_json::Value;

    #[test]
    fn test_toml_source_load() {
        let temp_dir = TempDir::new().unwrap();
        let toml_file = temp_dir.path().join("config.toml");
        
        // Create a test TOML file
        let mut file = File::create(&toml_file).unwrap();
        writeln!(file, "test_key = \"test_value\"").unwrap();
        writeln!(file, "another_key = 42").unwrap();
        
        let source = TomlFileSource::new(toml_file.to_str().unwrap(), false);
        let result = source.load();
        
        assert!(result.is_ok());
        let config = result.unwrap();
        
        assert_eq!(config.get("test_key"), Some(&Value::String("test_value".to_string())));
        assert_eq!(config.get("another_key"), Some(&Value::Number(42.into())));
    }
}