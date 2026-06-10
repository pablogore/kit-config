use std::fs::File;
use std::io::Write;
use tempfile::TempDir;

#[cfg(test)]
mod tests {
    use super::*;
    use kit_config::sources::DotenvSource;
    use kit_config::sources::ConfigurationSource;
    use serde_json::Value;

    #[test]
    fn test_dotenv_source_load() {
        let temp_dir = TempDir::new().unwrap();
        let dotenv_file = temp_dir.path().join(".env");
        
        // Create a test dotenv file
        let mut file = File::create(&dotenv_file).unwrap();
        writeln!(file, "TEST_KEY=TEST_VALUE").unwrap();
        writeln!(file, "ANOTHER_KEY=another_value").unwrap();
        
        let source = DotenvSource::new(dotenv_file.to_str().unwrap(), false);
        let result = source.load();
        
        assert!(result.is_ok());
        let config = result.unwrap();
        
        assert_eq!(config.get("TEST_KEY"), Some(&Value::String("TEST_VALUE".to_string())));
        assert_eq!(config.get("ANOTHER_KEY"), Some(&Value::String("another_value".to_string())));
    }
}