fn main() {
    // Simple test to verify TOML parsing works
    let toml_content = r#"
test_key = "test_value"
another_key = 42
"#;
    
    // Try to parse it directly with toml crate
    let parsed: toml::Value = toml::from_str(toml_content).unwrap();
    
    println!("Parsed TOML: {:?}", parsed);
    
    // Check if we can access values
    if let toml::Value::Table(table) = parsed {
        if let Some(value) = table.get("test_key") {
            println!("test_key value: {:?}", value);
        }
        if let Some(value) = table.get("another_key") {
            println!("another_key value: {:?}", value);
        }
    }
    
    println!("TOML parsing test completed successfully!");
}