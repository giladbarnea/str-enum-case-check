use std::path::Path;
use std::fs;
use regex::Regex;
use clap::Parser;

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

fn scan_directory(path: &Path) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    
    if path.is_dir() {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let entry_path = entry.path();
                if entry_path.is_dir() {
                    errors.extend(scan_directory(&entry_path));
                } else if let Some(ext) = entry_path.extension() {
                    if ext == "py" {
                        if let Ok(content) = fs::read_to_string(&entry_path) {
                            let file_errors = check_file(&content, entry_path.to_string_lossy().to_string());
                            errors.extend(file_errors);
                        }
                    }
                }
            }
        }
    } else if path.is_file() && path.extension().map_or(false, |ext| ext == "py") {
        if let Ok(content) = fs::read_to_string(path) {
            let file_errors = check_file(&content, path.to_string_lossy().to_string());
            errors.extend(file_errors);
        }
    }
    
    errors
}

fn check_file(content: &str, file_path: String) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    
    // Regex to find StrEnum class definitions
    let class_re = Regex::new(r"class\s+(\w+)\s*\(\s*(enum\.)?StrEnum\s*\)").unwrap();
    
    for (class_line_idx, line) in content.lines().enumerate() {
        if let Some(captures) = class_re.captures(line) {
            let enum_name = captures.get(1).unwrap().as_str().to_string();
            
            // Extract the class body
            let lines: Vec<&str> = content.lines().collect();
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
                            enum_name: enum_name.clone(),
                            member_name,
                            member_value,
                        });
                    }
                }
            }
        }
    }
    
    errors
}

fn parse_member_assignment(line: &str) -> Option<(String, String)> {
    // Split the line at the equals sign
    let parts: Vec<&str> = line.splitn(2, '=').collect();
    if parts.len() != 2 {
        return None;
    }
    
    // Extract the member name (trim whitespace)
    let member_name = parts[0].trim().to_string();
    
    // Extract the value - look for a string literal
    let value_part = parts[1].trim();
    
    // Check for quoted string - handles both single and double quotes
    if (value_part.starts_with('"') && value_part.contains("\"")) || 
       (value_part.starts_with('\'') && value_part.contains("'")) {
        let quote_char = if value_part.starts_with('"') { '"' } else { '\'' };
        let value_content = value_part
            .chars()
            .skip(1)  // Skip opening quote
            .take_while(|&c| c != quote_char)
            .collect::<String>();
        
        return Some((member_name, value_content));
    }
    
    None
}

fn extract_class_body(lines: &[&str], class_line_idx: usize) -> Vec<String> {
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
            
            body.push(line.to_string());
        }
        
        i += 1;
    }
    
    body
}