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
    match File::open(file_path) {
        Ok(mut file) => {
            if let Err(err) = file.read_to_string(&mut css_content) {
                eprintln!("Error reading file: {}", err);
                return;
            }
        }
        Err(err) => {
            eprintln!("Error opening file: {}", err);
            return;
        }
    }

    // Extract unique classes using regex
    let class_regex = Regex::new(r"\.([a-zA-Z0-9_-]+)").unwrap();
    let mut unique_classes = Vec::new();
    for class in class_regex.captures_iter(&css_content) {
        let class_name = &class[1];
        if !unique_classes.contains(&class_name.to_string()) {
            unique_classes.push(class_name.to_string());
        }
    }

    // Print the unique classes
    println!("Unique Classes:");
    for class in &unique_classes {
        println!("{}", class);
    }

    // Print the total count
    println!("Total Unique Classes: {}", unique_classes.len());
}
