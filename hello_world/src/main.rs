use std::fs;

fn main() {
    println!("Simple File System Demo");
    list_directory(".");
}

fn list_directory(path: &str) {
    match fs::read_dir(path) {
        Ok(entries) => {
            println!("Contents of {}:", path);
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(file_type) = entry.file_type() {
                        let name = entry.file_name().into_string().unwrap_or_else(|_| "Invalid UTF-8".to_string());
                        if file_type.is_dir() {
                            println!("  [DIR] {}", name);
                        } else {
                            println!("  [FILE] {}", name);
                        }
                    }
                }
            }
        }
        Err(e) => println!("Error reading directory: {}", e),
    }
}
