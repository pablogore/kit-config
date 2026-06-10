use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use tempfile::TempDir;
use dotenvy::from_read_iter;
use std::io::Cursor;

fn main() {
    let temp_dir = TempDir::new().unwrap();
    let dotenv_file = temp_dir.path().join(".env");
    
    // Create a test dotenv file
    let mut file = File::create(&dotenv_file).unwrap();
    writeln!(file, "TEST_KEY=TEST_VALUE").unwrap();
    writeln!(file, "ANOTHER_KEY=another_value").unwrap();
    
    let content = std::fs::read_to_string(&dotenv_file).unwrap();
    println!("File content: {:?}", content);
    
    let cursor = Cursor::new(content);
    let mut parsed = HashMap::new();
    for item in from_read_iter(cursor) {
        match item {
            Ok((key, value)) => {
                println!("Parsed key: {}, value: {}", key, value);
                parsed.insert(key, value);
            }
            Err(e) => {
                println!("Parse error: {:?}", e);
            }
        }
    }
    
    println!("Parsed map: {:?}", parsed);
}