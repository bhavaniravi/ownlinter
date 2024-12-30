// Rust Script
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;
use std::sync::{Arc, Mutex};
use rayon::prelude::*;
use serde::Deserialize;
// use std::env as sys;

#[derive(Deserialize)]
struct Config {
    line_length: usize,
}

fn load_config() -> Config {
    let config_content = fs::read_to_string("linter.json").expect("Config file 'linter.json' not found or unreadable.");
    serde_json::from_str(&config_content).expect("Error parsing 'linter.json'. Make sure it's valid JSON.")
}

fn find_py_files(dir: &Path) -> Vec<String> {
    let mut py_files = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir).expect("Unable to read directory.") {
            let entry = entry.expect("Error reading entry.");
            let path = entry.path();
            if path.is_dir() {
                py_files.extend(find_py_files(&path));
            } else if let Some(ext) = path.extension() {
                if ext == "py" {
                    py_files.push(path.to_string_lossy().to_string());
                }
            }
        }
    }
    py_files
}

fn scan_file(file_path: &str, line_length: usize) -> Vec<(usize, String)> {
    let mut errors = Vec::new();
    if let Ok(file) = fs::File::open(file_path) {
        let reader = io::BufReader::new(file);
        for (line_no, line) in reader.lines().enumerate() {
            if let Ok(line) = line {
                if line.len() > line_length {
                    errors.push((line_no + 1, line));
                }
            }
        }
    }
    errors
}

fn main() {
    // read path from sys args
    let path1 = std::env::args().nth(1).expect("No path provided.");

    let config = load_config();
    let line_length = config.line_length;
    let py_files = find_py_files(Path::new(&path1));

    let error_files = Arc::new(Mutex::new(Vec::new()));
    py_files.par_iter().for_each(|file_path| {
        // println!("Processing {} on thread {:?}", file_path, thread::current().id());
        let errors = scan_file(file_path, line_length);
        if !errors.is_empty() {
            let mut error_files = error_files.lock().unwrap();
            error_files.push((file_path.clone(), errors));
        }
    });

    let error_files = error_files.lock().unwrap();
    println!("\nScan Report:");
    println!("Files Scanned: {}", py_files.len());
    println!("Error Files: {}", error_files.len());
    println!("\n\n");
    // for (file, errors) in error_files.iter() {
    //     println!("\nFile: {}", file);
    //     for (line_no, line) in errors {
    //         println!("  Line {}: {}", line_no, line);
    //     }
    // }
}
