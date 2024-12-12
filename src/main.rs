use std::fs;
use std::env;
use std::path::Path;
use walkdir::{WalkDir, DirEntry};

fn should_skip(entry: &DirEntry, skip_dirs: &[String]) -> bool {
    entry.path()
        .file_name()
        .and_then(|n| n.to_str())
        .map_or(false, |name| skip_dirs.contains(&name.to_string()))
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <directory> [--skip dir1,dir2,dir3]", args[0]);
        std::process::exit(1);
    }
    
    let target_dir = Path::new(&args[1]);
    let skip_dirs: Vec<String> = args.iter()
        .position(|arg| arg == "--skip")
        .and_then(|i| args.get(i + 1))
        .map(|dirs| dirs.split(',').map(String::from).collect())
        .unwrap_or_default();
    
    for entry in WalkDir::new(target_dir)
        .into_iter()
        .filter_entry(|e| !should_skip(e, &skip_dirs))
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
    {
        let path = entry.path();
        let content = fs::read_to_string(path)?;
        let relative_path = path.strip_prefix(target_dir)
            .unwrap_or(path)
            .display()
            .to_string()
            .trim_start_matches('/')
            .to_string();
        let minified = minify(&content);
        println!("{} {}", relative_path, minified);
    }
    Ok(())
}

fn minify(content: &str) -> String {
    let mut result = String::with_capacity(content.len());
    let mut in_string = false;
    let mut in_char = false;
    let mut in_comment = false;
    let mut in_multiline_comment = false;
    let mut prev_char = None;
    let mut chars = content.chars().peekable();
    
    while let Some(c) = chars.next() {
        match c {
            '"' if !in_char && !in_comment && !in_multiline_comment => {
                if prev_char != Some('\\') { in_string = !in_string; }
                result.push(c);
            }
            '\'' if !in_string && !in_comment && !in_multiline_comment => {
                if prev_char != Some('\\') { in_char = !in_char; }
                result.push(c);
            }
            '/' if !in_string && !in_char && !in_comment && !in_multiline_comment => {
                if let Some(&next) = chars.peek() {
                    match next {
                        '/' => { chars.next(); in_comment = true; }
                        '*' => { chars.next(); in_multiline_comment = true; }
                        _ => result.push(c)
                    }
                } else {
                    result.push(c);
                }
            }
            '*' if in_multiline_comment => {
                if let Some('/') = chars.peek() {
                    chars.next();
                    in_multiline_comment = false;
                }
            }
            '\n' | '\r' => {
                if in_comment { in_comment = false; }
                else if in_string { result.push_str("\\n"); }
            }
            '\\' if in_string => {
                result.push(c);
                if let Some(&next) = chars.peek() {
                    if matches!(next, 'n' | 'r' | 't' | '\\' | '"') {
                        result.push(chars.next().unwrap());
                    }
                }
            }
            _ => {
                if !in_comment && !in_multiline_comment && (!c.is_whitespace() || in_string || in_char) {
                    result.push(c);
                }
            }
        }
        prev_char = Some(c);
    }

    // Collapse remaining whitespace to single spaces
    result.split_whitespace().collect::<Vec<_>>().join(" ")
}
