use std::path::Path;
use std::fs;
use regex::Regex;
use clap::Parser;
use rayon::prelude::*;
use walkdir::{WalkDir, DirEntry};
use lazy_static::lazy_static;
use std::collections::HashSet;
use serde::Deserialize;

lazy_static! {
    static ref CLASS_RE: Regex = Regex::new(r"class\s+(\w+)\s*\(\s*(?:.*?,\s*)?(enum\.)?StrEnum\s*(?:,\s*.*?)?\)").unwrap();
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Directory path to scan for Python files
    #[arg(short, long)]
    path: String,

    /// Comma-separated list of directory names to exclude
    /// Can also be configured via [tool.str-enum-case-check] section in pyproject.toml
    #[arg(short, long, value_delimiter = ',')]
    exclude: Option<Vec<String>>,
}

// Structures for parsing pyproject.toml
#[derive(Deserialize, Debug, Default)]
struct PyprojectToml {
    tool: Option<ToolSection>,
}

#[derive(Deserialize, Debug, Default)]
struct ToolSection {
    #[serde(rename = "str-enum-case-check")]
    str_enum_case_check: Option<StrEnumCaseCheckConfig>,
}

#[derive(Deserialize, Debug, Default)]
struct StrEnumCaseCheckConfig {
    exclude: Option<Vec<String>>,
}

struct ValidationError {
    file: String,
    line: usize,
    enum_name: String,
    member_name: String,
    member_value: String,
    error_type: ErrorType,
}

#[derive(Debug, PartialEq)]
enum ErrorType {
    // Member name and value have different cases (e.g., "a" = "A")
    NameValueMismatch,
    // Members within enum have inconsistent naming (e.g., "a" and "B" in same enum)
    InconsistentNaming,
    // Auto() with uppercase names (e.g., "A = auto()")
    InvalidAutoCase,
}

fn main() {
    let args = Args::parse();
    let path = Path::new(&args.path);
    
    if !path.exists() {
        eprintln!("Error: Path '{}' does not exist", args.path);
        std::process::exit(1);
    }
    
    // Try to read pyproject.toml for configuration
    let pyproject_excludes = read_pyproject_toml_config(path);
    
    // Merge command-line excludes with pyproject.toml excludes
    let mut all_excludes = HashSet::new();
    
    // Add pyproject.toml excludes if found
    if let Some(excludes) = pyproject_excludes {
        for exclude in excludes {
            all_excludes.insert(exclude);
        }
    }
    
    // Add command-line excludes
    if let Some(cli_excludes) = args.exclude {
        for exclude in cli_excludes {
            all_excludes.insert(exclude);
        }
    }
    
    let errors = scan_directory(path, &all_excludes);
    
    if errors.is_empty() {
        println!("No StrEnum inconsistencies found");
    } else {
        for error in errors {
            match error.error_type {
                ErrorType::NameValueMismatch => {
                    println!(
                        "{}:{}:1: StrEnum '{}' has member '{}' with inconsistent casing: value is '{}'",
                        error.file, error.line, error.enum_name, error.member_name, error.member_value
                    );
                },
                ErrorType::InconsistentNaming => {
                    println!(
                        "{}:{}:1: StrEnum '{}' has inconsistent member naming: '{}' differs from predominant style ({})",
                        error.file, error.line, error.enum_name, error.member_name, error.member_value
                    );
                },
                ErrorType::InvalidAutoCase => {
                    println!(
                        "{}:{}:1: StrEnum '{}' has uppercase member '{}' with auto() value which should be lowercase",
                        error.file, error.line, error.enum_name, error.member_name
                    );
                }
            }
        }
        std::process::exit(1);
    }
}

/// Read the pyproject.toml file from the target directory and extract str-enum-case-check settings
fn read_pyproject_toml_config(project_path: &Path) -> Option<Vec<String>> {
    let pyproject_path = project_path.join("pyproject.toml");
    
    if !pyproject_path.exists() {
        return None;
    }
    
    match fs::read_to_string(&pyproject_path) {
        Ok(content) => {
            match toml::from_str::<PyprojectToml>(&content) {
                Ok(config) => {
                    if let Some(tool) = &config.tool {
                        if let Some(str_enum_config) = &tool.str_enum_case_check {
                            return str_enum_config.exclude.clone();
                        }
                    }
                    None
                },
                Err(e) => {
                    eprintln!("Warning: Failed to parse pyproject.toml: {}", e);
                    None
                }
            }
        },
        Err(e) => {
            eprintln!("Warning: Failed to read pyproject.toml: {}", e);
            None
        }
    }
}

fn should_skip_dir(entry: &DirEntry, custom_excludes: &HashSet<String>) -> bool {
    let file_name = entry.file_name().to_string_lossy();
    
    // Check custom excludes first
    if custom_excludes.contains(file_name.as_ref()) {
        return true;
    }
    
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

/// Check if a file should be excluded based on the custom excludes list
fn should_exclude_file(path: &Path, custom_excludes: &HashSet<String>) -> bool {
    // Check if the filename is in the excludes list
    if let Some(file_name) = path.file_name() {
        let file_name_str = file_name.to_string_lossy();
        if custom_excludes.contains(file_name_str.as_ref()) {
            return true;
        }
    }
    
    // Also check if any parent directory is in the excludes list
    for ancestor in path.ancestors().skip(1) {  // Skip the file itself
        if let Some(dir_name) = ancestor.file_name() {
            let dir_name_str = dir_name.to_string_lossy();
            if custom_excludes.contains(dir_name_str.as_ref()) {
                return true;
            }
        }
    }
    
    false
}

fn scan_directory(path: &Path, custom_excludes: &HashSet<String>) -> Vec<ValidationError> {
    WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| !e.path().is_dir() || !should_skip_dir(e, custom_excludes))
        .filter_map(Result::ok)
        .filter(|entry| {
            let path = entry.path();
            is_python_file(path) && !should_exclude_file(path, custom_excludes)
        })
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
            
            // Track case consistency within the enum
            let mut case_state: Option<bool> = None; // None = unset, Some(true) = uppercase, Some(false) = lowercase
            let mut members_info = Vec::new();
            let mut uppercase_count = 0;
            let mut lowercase_count = 0;
            
            // First pass: collect all member information
            for (i, body_line) in class_body.iter().enumerate() {
                let line_idx = class_line_idx + i + 1;
                
                // Skip any non-assignment lines or commented lines
                let trimmed_line = body_line.trim();
                if !trimmed_line.contains('=') || trimmed_line.starts_with('#') {
                    continue;
                }
                
                // Handle auto() case
                let is_auto = body_line.contains("auto()") || body_line.contains("enum.auto()");
                
                // Parse the line to extract member name
                if let Some((member_name, member_value_opt)) = if is_auto {
                    let parts: Vec<&str> = body_line.splitn(2, '=').collect();
                    if parts.len() > 0 {
                        Some((parts[0].trim(), None))
                    } else {
                        None
                    }
                } else {
                    parse_member_assignment(body_line).map(|(name, value)| (name, Some(value)))
                } {
                    // Determine if this member name is uppercase or lowercase
                    let is_uppercase = member_name.chars().next().map_or(false, |c| c.is_uppercase());
                    
                    // Update counts
                    if is_uppercase {
                        uppercase_count += 1;
                    } else {
                        lowercase_count += 1;
                    }
                    
                    // Check for auto() with uppercase name - always invalid
                    if is_auto && is_uppercase {
                        errors.push(ValidationError {
                            file: file_path.clone(),
                            line: line_idx + 1,
                            enum_name: enum_name.to_string(),
                            member_name: member_name.to_string(),
                            member_value: "auto()".to_string(),
                            error_type: ErrorType::InvalidAutoCase,
                        });
                    }
                    
                    // Update case state if this is a non-auto member with a direct string value
                    if !is_auto && member_value_opt.is_some() {
                        // Track the casing pattern for the enum
                        if case_state.is_none() {
                            case_state = Some(is_uppercase);
                        } else if case_state != Some(is_uppercase) {
                            // We've found inconsistency in member name casing
                            // Determine the predominant style
                            let predominant_style = if uppercase_count > lowercase_count { 
                                "UPPERCASE" 
                            } else { 
                                "lowercase" 
                            };
                            
                            errors.push(ValidationError {
                                file: file_path.clone(),
                                line: line_idx + 1,
                                enum_name: enum_name.to_string(),
                                member_name: member_name.to_string(),
                                member_value: predominant_style.to_string(),
                                error_type: ErrorType::InconsistentNaming,
                            });
                        }
                    }
                    
                    // Save for second pass
                    members_info.push((line_idx, member_name.to_string(), member_value_opt.map(ToString::to_string), is_auto));
                }
            }
            
            // Second pass: check for case differences between names and values
            for (line_idx, member_name, member_value_opt, is_auto) in &members_info {
                if *is_auto {
                    continue; // Already checked in first pass
                }
                
                if let Some(member_value) = member_value_opt {
                    // Skip if name and value are completely different strings
                    if member_name.to_lowercase() != member_value.to_lowercase() {
                        continue;
                    }
                    
                    // Check for case mismatch between name and value
                    if member_name != member_value {
                        errors.push(ValidationError {
                            file: file_path.clone(),
                            line: *line_idx + 1,
                            enum_name: enum_name.to_string(),
                            member_name: member_name.clone(),
                            member_value: member_value.clone(),
                            error_type: ErrorType::NameValueMismatch,
                        });
                    }
                }
            }
            
            // Check for auto() members inconsistent with string value members
            if let Some(is_uppercase_case) = case_state {
                for (line_idx, member_name, _, is_auto) in &members_info {
                    if *is_auto {
                        let member_first_char_uppercase = member_name.chars().next().map_or(false, |c| c.is_uppercase());
                        if member_first_char_uppercase != is_uppercase_case {
                            errors.push(ValidationError {
                                file: file_path.clone(),
                                line: *line_idx + 1,
                                enum_name: enum_name.to_string(),
                                member_name: member_name.clone(),
                                member_value: "auto()".to_string(),
                                error_type: ErrorType::InvalidAutoCase,
                            });
                        }
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
    
    // Handle string concatenation like "H" + ello or f"w{orld}"
    // Look for patterns that indicate it's a string operation
    if value_part.contains('"') || value_part.contains('\'') || 
       value_part.contains("f\"") || value_part.contains("f'") ||
       value_part.contains(" + ") {
        
        // Check first character of the expression for case hint
        // For expressions like "H" + ello, check the first quoted character
        if let Some(first_quote_idx) = value_part.find('"').or_else(|| value_part.find('\'')) {
            if first_quote_idx + 1 < value_part.len() {
                let char_after_quote = value_part[first_quote_idx + 1..].chars().next().unwrap_or(' ');
                if char_after_quote.is_alphabetic() {
                    // Return the member name and just the initial character for case checking
                    return Some((member_name, &value_part[first_quote_idx + 1..first_quote_idx + 2]));
                }
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