use std::path::Path;
use std::fs;
use regex::Regex;
use clap::Parser;
use rayon::prelude::*;
use walkdir::{WalkDir, DirEntry};
use lazy_static::lazy_static;

lazy_static! {
    static ref CLASS_RE: Regex = Regex::new(r"class\s+(\w+)\s*\(\s*(enum\.)?StrEnum\s*\)").unwrap();
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Directory path to scan for Python files
    #[arg(short, long)]
    path: String,
}

struct ValidationError {
    file: String,
    line: usize,
    enum_name: String,
    member_name: String,
    member_value: String,
}

fn main() {
    let args = Args::parse();
    let path = Path::new(&args.path);
    
    if !path.exists() {
        eprintln!("Error: Path '{}' does not exist", args.path);
        std::process::exit(1);
    }
    
    let errors = scan_directory(path);
    
    if errors.is_empty() {
        println!("No StrEnum inconsistencies found");
    } else {
        for error in errors {
            println!(
                "{}:{}:1: StrEnum '{}' has member '{}' with inconsistent casing: value is '{}'",
                error.file, error.line, error.enum_name, error.member_name, error.member_value
            );
        }
        std::process::exit(1);
    }
}

fn should_skip_dir(entry: &DirEntry) -> bool {
    let file_name = entry.file_name().to_string_lossy();
    
    // Skip directories starting with dot or underscore
    file_name.starts_with('.') || 
    file_name.starts_with('_') || 
    // Skip specific directory names
    file_name == "frontend" || 
    file_name == "build" ||
    file_name == "dist" ||
    // Skip egg-info directories
    file_name.ends_with(".egg-info") ||
    // Skip common Python virtual environment directories
    file_name == "venv" ||
    file_name == "env" ||
    file_name == "__pycache__"
}

fn is_python_file(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        let ext_str = ext.to_string_lossy();
        // Only process .py files, skip .pyc, .pyi, .ipynb, etc.
        ext_str == "py" && 
        !path.to_string_lossy().ends_with(".py.typed")
    } else {
        false
    }
}

fn scan_directory(path: &Path) -> Vec<ValidationError> {
    WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| !e.path().is_dir() || !should_skip_dir(e))
        .filter_map(Result::ok)
        .filter(|entry| is_python_file(entry.path()))
        .par_bridge() // Process files in parallel
        .filter_map(|entry| {
            let path = entry.path();
            fs::read_to_string(path)
                .ok()
                .map(|content| check_file(&content, path.to_string_lossy().to_string()))
        })
        .flatten()
        .collect()
}

fn check_file(content: &str, file_path: String) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    
    for (class_line_idx, line) in lines.iter().enumerate() {
        if let Some(captures) = CLASS_RE.captures(line) {
            let enum_name = captures.get(1).unwrap().as_str();
            
            // Extract the class body
            let class_body = extract_class_body(&lines, class_line_idx);
            
            // Process the class body
            for (i, body_line) in class_body.iter().enumerate() {
                let line_idx = class_line_idx + i + 1;
                
                // Skip any non-assignment lines and auto() calls
                if !body_line.contains('=') || 
                   body_line.contains("auto()") || 
                   body_line.contains("enum.auto()") {
                    continue;
                }
                
                // Parse the line manually instead of using regex
                if let Some((member_name, member_value)) = parse_member_assignment(body_line) {
                    
                    // Check if member_name and member_value have the same casing
                    if member_name != member_value && member_name.to_lowercase() == member_value.to_lowercase() {
                        errors.push(ValidationError {
                            file: file_path.clone(),
                            line: line_idx + 1,
                            enum_name: enum_name.to_string(),
                            member_name: member_name.to_string(),
                            member_value: member_value.to_string(),
                        });
                    }
                }
            }
        }
    }
    
    errors
}

fn parse_member_assignment(line: &str) -> Option<(&str, &str)> {
    // Split the line at the equals sign
    let parts: Vec<&str> = line.splitn(2, '=').collect();
    if parts.len() != 2 {
        return None;
    }
    
    // Extract the member name (trim whitespace)
    let member_name = parts[0].trim();
    
    // Extract the value - look for a string literal
    let value_part = parts[1].trim();
    
    // Check for quoted string - handles both single and double quotes
    if (value_part.starts_with('"') && value_part.contains('"')) || 
       (value_part.starts_with('\'') && value_part.contains('\'')) {
        let quote_char = if value_part.starts_with('"') { '"' } else { '\'' };
        
        // Find the content between quotes more efficiently
        if let Some(first_quote) = value_part.find(quote_char) {
            if let Some(second_quote) = value_part[first_quote + 1..].find(quote_char) {
                let value_content = &value_part[first_quote + 1..first_quote + second_quote + 1];
                return Some((member_name, value_content));
            }
        }
    }
    
    None
}

fn extract_class_body<'a>(lines: &'a [&str], class_line_idx: usize) -> Vec<&'a str> {
    let mut body = Vec::new();
    let mut i = class_line_idx + 1;
    let mut indent_level = None;
    
    while i < lines.len() {
        let line = lines[i];
        if line.trim().is_empty() {
            i += 1;
            continue;
        }
        
        // Calculate indentation level
        let current_indent = line.len() - line.trim_start().len();
        
        // If this is the first non-empty line after class definition,
        // set the expected indentation level
        if indent_level.is_none() {
            indent_level = Some(current_indent);
        }
        
        // If indentation is less than or equal to the class indent, 
        // we've reached the end of the class
        if let Some(expected_indent) = indent_level {
            if current_indent < expected_indent {
                break;
            }
            
            body.push(line);
        }
        
        i += 1;
    }
    
    body
}