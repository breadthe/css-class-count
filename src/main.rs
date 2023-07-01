use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;
use regex::Regex;

fn main() {
    // Get the CSS file path from command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide the path to a CSS file.");
        return;
    }
    let file_path = &args[1];

    // Read the CSS file
    let mut css_content = String::new();
    let file_size_bytes: u64;
    match File::open(file_path) {
        Ok(mut file) => {
            if let Err(err) = file.read_to_string(&mut css_content) {
                eprintln!("Error reading file: {}", err);
                return;
            }
            file_size_bytes = match file.metadata() {
                Ok(metadata) => metadata.len(),
                Err(_) => 0,
            };
        }
        Err(err) => {
            eprintln!("Error opening file: {}", err);
            return;
        }
    }

    // Extract unique classes using regex and count them
    let class_regex = Regex::new(r"\.([a-zA-Z0-9_-]+)").unwrap();
    let mut class_counts = HashMap::new();
    for class in class_regex.captures_iter(&css_content) {
        let class_name = &class[1];
        let count = class_counts.entry(class_name.to_string()).or_insert(0);
        *count += 1;
    }

    // Print the unique classes and their counts
    println!("Class Counts:");
    for (class_name, count) in &class_counts {
        println!("{}: {}", class_name, count);
    }

    // Print the total count and file details
    println!("Total Unique Classes: {}", class_counts.len());
    println!("CSS File: {}", file_path);
    println!("File Size: {} bytes", file_size_bytes);
}
