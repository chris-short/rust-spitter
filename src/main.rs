use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

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
            default_skip_dirs: vec![
                "__pycache__".to_string(),
                "venv".to_string(),
                ".env".to_string(),
                "dist".to_string(),
            ],
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
            default_skip_dirs: vec![
                "node_modules".to_string(),
                "dist".to_string(),
                "build".to_string(),
            ],
            line_comment: "//".to_string(),
            block_comment_start: "/*".to_string(),
            block_comment_end: "*/".to_string(),
        }),

        // 4. C++
        "cpp" | "c++" => Some(Language {
            extensions: vec![
                "cpp".to_string(),
                "hpp".to_string(),
                "cc".to_string(),
                "hh".to_string(),
                "cxx".to_string(),
                "hxx".to_string(),
            ],
            default_skip_dirs: vec!["build".to_string(), "obj".to_string(), "bin".to_string()],
            line_comment: "//".to_string(),
            block_comment_start: "/*".to_string(),
            block_comment_end: "*/".to_string(),
        }),

        // 5. C#
        "csharp" | "c#" => Some(Language {
            extensions: vec!["cs".to_string()],
            default_skip_dirs: vec![
                "bin".to_string(),
                "obj".to_string(),
                "Debug".to_string(),
                "Release".to_string(),
            ],
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
            default_skip_dirs: vec![
                "node_modules".to_string(),
                "dist".to_string(),
                "build".to_string(),
            ],
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
            extensions: vec![
                "groovy".to_string(),
                "gvy".to_string(),
                "gy".to_string(),
                "gsh".to_string(),
            ],
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
    entry
        .path()
        .file_name()
        .and_then(|n| n.to_str())
        .is_some_and(|name| skip_dirs.contains(&name.to_string()))
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!(
            "Usage: {} <directory> --lang <language> [--skip dir1,dir2,dir3]",
            args[0]
        );
        eprintln!("\nSupported languages:");
        eprintln!("  rust, go, c, cpp, python, javascript, typescript");
        std::process::exit(1);
    }

    let target_dir = Path::new(&args[1]);

    // Get language from arguments
    let lang = args
        .iter()
        .position(|arg| arg == "--lang")
        .and_then(|i| args.get(i + 1))
        .unwrap_or_else(|| {
            eprintln!("Error: --lang argument is required");
            std::process::exit(1);
        });

    let language_config = get_language_config(lang)
        .unwrap_or_else(|| {
            eprintln!("Error: unsupported language '{}'", lang);
            eprintln!("Supported: rust, go, c, cpp, python, javascript, typescript, java, csharp, php, ruby, swift, kotlin, scala, perl, dart, groovy, julia, haskell, shell, lua, r, matlab, vbnet, objective-c");
            std::process::exit(1);
        });

    // Combine default and user-specified skip directories
    let mut skip_dirs = language_config.default_skip_dirs;
    if let Some(pos) = args.iter().position(|arg| arg == "--skip") {
        if let Some(user_dirs) = args.get(pos + 1) {
            skip_dirs.extend(user_dirs.split(',').map(String::from));
        }
    }

    // Create extension filter
    let valid_extensions: HashMap<_, _> = language_config
        .extensions
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
                .is_some_and(|ext| valid_extensions.contains_key(ext))
        })
    {
        let path = entry.path();
        let relative_path = path
            .strip_prefix(target_dir)
            .unwrap_or(path)
            .display()
            .to_string()
            .trim_start_matches('/')
            .to_string();
        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Warning: skipping {}: {}", relative_path, e);
                continue;
            }
        };
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

fn starts_with_at(chars: &[char], pos: usize, pattern: &[char]) -> bool {
    if pattern.is_empty() || pos + pattern.len() > chars.len() {
        return false;
    }
    chars[pos..pos + pattern.len()] == *pattern
}

fn minify(
    content: &str,
    line_comment: &str,
    block_comment_start: &str,
    block_comment_end: &str,
) -> String {
    let chars: Vec<char> = content.chars().collect();
    let lc: Vec<char> = line_comment.chars().collect();
    let bcs: Vec<char> = block_comment_start.chars().collect();
    let bce: Vec<char> = block_comment_end.chars().collect();
    let n = chars.len();

    let mut result = String::with_capacity(content.len());
    let mut in_string = false;
    let mut in_char = false;
    let mut in_line_comment = false;
    let mut in_block_comment = false;
    let mut i = 0;

    while i < n {
        let c = chars[i];

        // End line comment on newline
        if in_line_comment {
            if c == '\n' || c == '\r' {
                in_line_comment = false;
            }
            i += 1;
            continue;
        }

        // End block comment when we see the closing sequence
        if in_block_comment {
            if starts_with_at(&chars, i, &bce) {
                in_block_comment = false;
                i += bce.len();
            } else {
                i += 1;
            }
            continue;
        }

        // Escape sequences inside string or char literals
        if c == '\\' && (in_string || in_char) {
            result.push(c);
            if i + 1 < n {
                result.push(chars[i + 1]);
                i += 2;
            } else {
                i += 1;
            }
            continue;
        }

        // String delimiter
        if c == '"' && !in_char {
            in_string = !in_string;
            result.push(c);
            i += 1;
            continue;
        }

        // Char/single-quote delimiter
        if c == '\'' && !in_string {
            in_char = !in_char;
            result.push(c);
            i += 1;
            continue;
        }

        // Inside a string or char literal — preserve content
        if in_string || in_char {
            if c == '\n' || c == '\r' {
                result.push_str("\\n");
            } else {
                result.push(c);
            }
            i += 1;
            continue;
        }

        // Detect line comment start (checked before block comment so // wins over /*)
        if starts_with_at(&chars, i, &lc) {
            in_line_comment = true;
            i += lc.len();
            continue;
        }

        // Detect block comment start
        if starts_with_at(&chars, i, &bcs) {
            in_block_comment = true;
            i += bcs.len();
            continue;
        }

        // Normal character outside all comments and literals
        if c != '\n' && c != '\r' && !c.is_whitespace() {
            result.push(c);
        }
        i += 1;
    }

    result.split_whitespace().collect::<Vec<_>>().join(" ")
}
