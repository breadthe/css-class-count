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
    println!("Class Counts");
    println!("----------------------------------------");
    for (class_name, count) in &class_counts {
        println!("{}: {}", class_name, count);
    }

    // Print the total count and file details
    println!("----------------------------------------");
    println!("Total Unique Classes: {}", class_counts.len());
    println!("Full Path: {}", file_path);
    println!("CSS File: {}", get_file_name(file_path));
    println!("File Size: {} ({} bytes)", format_file_size(file_size_bytes), file_size_bytes);
}

fn get_file_name(file_path: &str) -> &str {
    if let Some(file_name) = file_path.rsplit('/').next() {
        file_name
    } else {
        file_path
    }
}

fn format_file_size(file_size_bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = 1024 * KB;
    const GB: u64 = 1024 * MB;
    const TB: u64 = 1024 * GB;

    if file_size_bytes >= TB {
        format!("{:.2} TB", file_size_bytes as f64 / TB as f64)
    } else if file_size_bytes >= GB {
        format!("{:.2} GB", file_size_bytes as f64 / GB as f64)
    } else if file_size_bytes >= MB {
        format!("{:.2} MB", file_size_bytes as f64 / MB as f64)
    } else if file_size_bytes >= KB {
        format!("{:.2} KB", file_size_bytes as f64 / KB as f64)
    } else {
        format!("{} bytes", file_size_bytes)
    }
}
