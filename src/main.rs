use std::fs;
use std::env;
use std::path::Path;
use std::collections::HashMap;
use walkdir::{WalkDir, DirEntry};

struct Language {
    extensions: Vec<String>,
    default_skip_dirs: Vec<String>,
    line_comment: String,
    block_comment_start: String,
    block_comment_end: String,
}

fn get_language_config(lang: &str) -> Option<Language> {
    match lang.to_lowercase().as_str() {
        // 1. Python
        "python" => Some(Language {
            extensions: vec!["py".to_string(), "pyw".to_string()],
            default_skip_dirs: vec!["__pycache__".to_string(), "venv".to_string(), ".env".to_string(), "dist".to_string()],
            line_comment: "#".to_string(),
            block_comment_start: "'''".to_string(),
            block_comment_end: "'''".to_string(),
        }),
        
        // 2. Java
        "java" => Some(Language {
            extensions: vec!["java".to_string()],
            default_skip_dirs: vec!["target".to_string(), "build".to_string(), "out".to_string()],
            line_comment: "//".to_string(),
            block_comment_start: "/*".to_string(),
            block_comment_end: "*/".to_string(),
        }),
        
        // 3. JavaScript
        "javascript" | "js" => Some(Language {
            extensions: vec!["js".to_string(), "jsx".to_string(), "mjs".to_string()],
            default_skip_dirs: vec!["node_modules".to_string(), "dist".to_string(), "build".to_string()],
            line_comment: "//".to_string(),
            block_comment_start: "/*".to_string(),
            block_comment_end: "*/".to_string(),
        }),
        
        // 4. C++
        "cpp" | "c++" => Some(Language {
            extensions: vec!["cpp".to_string(), "hpp".to_string(), "cc".to_string(), "hh".to_string(), "cxx".to_string(), "hxx".to_string()],
            default_skip_dirs: vec!["build".to_string(), "obj".to_string(), "bin".to_string()],
            line_comment: "//".to_string(),
            block_comment_start: "/*".to_string(),
            block_comment_end: "*/".to_string(),
        }),
        
        // 5. C#
        "csharp" | "c#" => Some(Language {
            extensions: vec!["cs".to_string()],
            default_skip_dirs: vec!["bin".to_string(), "obj".to_string(), "Debug".to_string(), "Release".to_string()],
            line_comment: "//".to_string(),
            block_comment_start: "/*".to_string(),
            block_comment_end: "*/".to_string(),
        }),
        
        // 6. PHP
        "php" => Some(Language {
            extensions: vec!["php".to_string()],
            default_skip_dirs: vec!["vendor".to_string(), "cache".to_string()],
            line_comment: "//".to_string(),
            block_comment_start: "/*".to_string(),
            block_comment_end: "*/".to_string(),
        }),
        
        // 7. Ruby
        "ruby" => Some(Language {
            extensions: vec!["rb".to_string()],
            default_skip_dirs: vec!["vendor".to_string(), "tmp".to_string(), "log".to_string()],
            line_comment: "#".to_string(),
            block_comment_start: "=begin".to_string(),
            block_comment_end: "=end".to_string(),
        }),
        
        // 8. Swift
        "swift" => Some(Language {
            extensions: vec!["swift".to_string()],
            default_skip_dirs: vec![".build".to_string(), "Pods".to_string()],
            line_comment: "//".to_string(),
            block_comment_start: "/*".to_string(),
            block_comment_end: "*/".to_string(),
        }),
        
        // 9. TypeScript
        "typescript" | "ts" => Some(Language {
            extensions: vec!["ts".to_string(), "tsx".to_string()],
            default_skip_dirs: vec!["node_modules".to_string(), "dist".to_string(), "build".to_string()],
            line_comment: "//".to_string(),
            block_comment_start: "/*".to_string(),
            block_comment_end: "*/".to_string(),
        }),
        
        // 10. Kotlin
        "kotlin" | "kt" => Some(Language {
            extensions: vec!["kt".to_string(), "kts".to_string()],
            default_skip_dirs: vec!["build".to_string(), "out".to_string()],
            line_comment: "//".to_string(),
            block_comment_start: "/*".to_string(),
            block_comment_end: "*/".to_string(),
        }),
        
        // 11. Go
        "go" => Some(Language {
            extensions: vec!["go".to_string()],
            default_skip_dirs: vec!["vendor".to_string(), "bin".to_string()],
            line_comment: "//".to_string(),
            block_comment_start: "/*".to_string(),
            block_comment_end: "*/".to_string(),
        }),
        
        // 12. Rust
        "rust" => Some(Language {
            extensions: vec!["rs".to_string()],
            default_skip_dirs: vec!["target".to_string()],
            line_comment: "//".to_string(),
            block_comment_start: "/*".to_string(),
            block_comment_end: "*/".to_string(),
        }),
        
        // 13. R
        "r" => Some(Language {
            extensions: vec!["r".to_string(), "R".to_string()],
            default_skip_dirs: vec!["renv".to_string()],
            line_comment: "#".to_string(),
            block_comment_start: "".to_string(), // R doesn't have traditional block comments
            block_comment_end: "".to_string(),
        }),
        
        // 14. MATLAB
        "matlab" => Some(Language {
            extensions: vec!["m".to_string()],
            default_skip_dirs: vec!["bin".to_string()],
            line_comment: "%".to_string(),
            block_comment_start: "%{".to_string(),
            block_comment_end: "%}".to_string(),
        }),
        
        // 15. VB.NET
        "vbnet" | "vb" => Some(Language {
            extensions: vec!["vb".to_string()],
            default_skip_dirs: vec!["bin".to_string(), "obj".to_string()],
            line_comment: "'".to_string(),
            block_comment_start: "".to_string(), // VB.NET uses line comments primarily
            block_comment_end: "".to_string(),
        }),
        
        // 16. Scala
        "scala" => Some(Language {
            extensions: vec!["scala".to_string()],
            default_skip_dirs: vec!["target".to_string(), "project/target".to_string()],
            line_comment: "//".to_string(),
            block_comment_start: "/*".to_string(),
            block_comment_end: "*/".to_string(),
        }),
        
        // 17. Perl
        "perl" => Some(Language {
            extensions: vec!["pl".to_string(), "pm".to_string()],
            default_skip_dirs: vec!["blib".to_string(), "_build".to_string()],
            line_comment: "#".to_string(),
            block_comment_start: "=pod".to_string(),
            block_comment_end: "=cut".to_string(),
        }),
        
        // 18. Dart
        "dart" => Some(Language {
            extensions: vec!["dart".to_string()],
            default_skip_dirs: vec!["build".to_string(), ".dart_tool".to_string()],
            line_comment: "//".to_string(),
            block_comment_start: "/*".to_string(),
            block_comment_end: "*/".to_string(),
        }),
        
        // 19. Objective-C
        "objective-c" | "objc" => Some(Language {
            extensions: vec!["m".to_string(), "mm".to_string()],
            default_skip_dirs: vec!["build".to_string(), "DerivedData".to_string()],
            line_comment: "//".to_string(),
            block_comment_start: "/*".to_string(),
            block_comment_end: "*/".to_string(),
        }),
        
        // 20. Groovy
        "groovy" => Some(Language {
            extensions: vec!["groovy".to_string(), "gvy".to_string(), "gy".to_string(), "gsh".to_string()],
            default_skip_dirs: vec!["target".to_string(), "build".to_string()],
            line_comment: "//".to_string(),
            block_comment_start: "/*".to_string(),
            block_comment_end: "*/".to_string(),
        }),
        
        // 21. Julia
        "julia" => Some(Language {
            extensions: vec!["jl".to_string()],
            default_skip_dirs: vec!["docs/build".to_string()],
            line_comment: "#".to_string(),
            block_comment_start: "#=".to_string(),
            block_comment_end: "=#".to_string(),
        }),
        
        // 22. Haskell
        "haskell" => Some(Language {
            extensions: vec!["hs".to_string(), "lhs".to_string()],
            default_skip_dirs: vec!["dist".to_string(), ".stack-work".to_string()],
            line_comment: "--".to_string(),
            block_comment_start: "{-".to_string(),
            block_comment_end: "-}".to_string(),
        }),
        
        // 23. Shell/Bash
        "shell" | "bash" => Some(Language {
            extensions: vec!["sh".to_string(), "bash".to_string()],
            default_skip_dirs: vec!["tmp".to_string()],
            line_comment: "#".to_string(),
            block_comment_start: "".to_string(), // Shell typically uses only line comments
            block_comment_end: "".to_string(),
        }),
        
        // 24. Lua
        "lua" => Some(Language {
            extensions: vec!["lua".to_string()],
            default_skip_dirs: vec!["bin".to_string()],
            line_comment: "--".to_string(),
            block_comment_start: "--[[".to_string(),
            block_comment_end: "]]".to_string(),
        }),
        
        // 25. C
        "c" => Some(Language {
            extensions: vec!["c".to_string(), "h".to_string()],
            default_skip_dirs: vec!["build".to_string(), "obj".to_string(), "bin".to_string()],
            line_comment: "//".to_string(),
            block_comment_start: "/*".to_string(),
            block_comment_end: "*/".to_string(),
        }),
        
        _ => None,
    }
}

fn should_skip(entry: &DirEntry, skip_dirs: &[String]) -> bool {
    entry.path()
        .file_name()
        .and_then(|n| n.to_str())
        .map_or(false, |name| skip_dirs.contains(&name.to_string()))
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        eprintln!("Usage: {} <directory> --lang <language> [--skip dir1,dir2,dir3]", args[0]);
        eprintln!("\nSupported languages:");
        eprintln!("  rust, go, c, cpp, python, javascript, typescript");
        std::process::exit(1);
    }
    
    let target_dir = Path::new(&args[1]);
    
    // Get language from arguments
    let lang = args.iter()
        .position(|arg| arg == "--lang")
        .and_then(|i| args.get(i + 1))
        .expect("--lang argument is required");
    
    let language_config = get_language_config(lang)
        .expect("Unsupported language");
    
    // Combine default and user-specified skip directories
    let mut skip_dirs = language_config.default_skip_dirs;
    if let Some(pos) = args.iter().position(|arg| arg == "--skip") {
        if let Some(user_dirs) = args.get(pos + 1) {
            skip_dirs.extend(user_dirs.split(',').map(String::from));
        }
    }
    
    // Create extension filter
    let valid_extensions: HashMap<_, _> = language_config.extensions
        .into_iter()
        .map(|ext| (ext, true))
        .collect();
    
    for entry in WalkDir::new(target_dir)
        .into_iter()
        .filter_entry(|e| !should_skip(e, &skip_dirs))
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|ext| ext.to_str())
                .map_or(false, |ext| valid_extensions.contains_key(ext))
        })
    {
        let path = entry.path();
        let content = fs::read_to_string(path)?;
        let relative_path = path.strip_prefix(target_dir)
            .unwrap_or(path)
            .display()
            .to_string()
            .trim_start_matches('/')
            .to_string();
        let minified = minify(
            &content,
            &language_config.line_comment,
            &language_config.block_comment_start,
            &language_config.block_comment_end,
        );
        println!("{} {}", relative_path, minified);
    }
    Ok(())
}

fn minify(
    content: &str,
    line_comment: &str,
    block_comment_start: &str,
    block_comment_end: &str,
) -> String {
    let mut result = String::with_capacity(content.len());
    let mut in_string = false;
    let mut in_char = false;
    let mut in_line_comment = false;
    let mut in_block_comment = false;
    let mut prev_char = None;
    let mut chars = content.chars().peekable();
    
    while let Some(c) = chars.next() {
        match c {
            '"' if !in_char && !in_line_comment && !in_block_comment => {
                if prev_char != Some('\\') { in_string = !in_string; }
                result.push(c);
            }
            '\'' if !in_string && !in_line_comment && !in_block_comment => {
                if prev_char != Some('\\') { in_char = !in_char; }
                result.push(c);
            }
            _ if !in_string && !in_char && !in_line_comment && !in_block_comment => {
                // Check for line comment start
                if c == line_comment.chars().next().unwrap() {
                    let mut is_line_comment = true;
                    for expected in line_comment.chars().skip(1) {
                        if chars.next() != Some(expected) {
                            is_line_comment = false;
                            break;
                        }
                    }
                    if is_line_comment {
                        in_line_comment = true;
                        continue;
                    }
                }
                
                // Check for block comment start
                if c == block_comment_start.chars().next().unwrap() {
                    let mut is_block_start = true;
                    for expected in block_comment_start.chars().skip(1) {
                        if chars.next() != Some(expected) {
                            is_block_start = false;
                            break;
                        }
                    }
                    if is_block_start {
                        in_block_comment = true;
                        continue;
                    }
                }
                
                // Check for block comment end
                if in_block_comment && c == block_comment_end.chars().next().unwrap() {
                    let mut is_block_end = true;
                    for expected in block_comment_end.chars().skip(1) {
                        if chars.next() != Some(expected) {
                            is_block_end = false;
                            break;
                        }
                    }
                    if is_block_end {
                        in_block_comment = false;
                        continue;
                    }
                }
                
                if !c.is_whitespace() {
                    result.push(c);
                }
            }
            '\n' | '\r' => {
                if in_line_comment {
                    in_line_comment = false;
                } else if in_string {
                    result.push_str("\\n");
                }
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
                if !in_line_comment && !in_block_comment && (!c.is_whitespace() || in_string || in_char) {
                    result.push(c);
                }
            }
        }
        prev_char = Some(c);
    }

    result.split_whitespace().collect::<Vec<_>>().join(" ")
}