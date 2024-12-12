# rust-spitter

A universal code minifier that supports multiple programming languages. This tool finds all relevant source files in a directory, minifies them, and outputs them in a standardized format.

## Basic Usage

```bash
cargo run -- <directory> --lang <language> [--skip dir1,dir2,dir3]
```

![example output](example.webp)

## Language Examples

### Web Development

```bash
# JavaScript
cargo run -- /path/to/project --lang javascript --skip node_modules,dist,coverage

# TypeScript
cargo run -- /path/to/project --lang typescript --skip node_modules,dist

# PHP
cargo run -- /path/to/project --lang php --skip vendor,cache
```

### Systems Programming

```bash
# Rust
cargo run -- /path/to/project --lang rust --skip target

# C
cargo run -- /path/to/project --lang c --skip build,obj

# C++
cargo run -- /path/to/project --lang cpp --skip build,obj

# Go
cargo run -- /path/to/project --lang go --skip vendor
```

### Mobile Development

```bash
# Swift
cargo run -- /path/to/project --lang swift --skip .build,Pods

# Kotlin
cargo run -- /path/to/project --lang kotlin --skip build

# Objective-C
cargo run -- /path/to/project --lang objective-c --skip build,DerivedData
```

### General Purpose

```bash
# Python
cargo run -- /path/to/project --lang python --skip __pycache__,venv,dist

# Java
cargo run -- /path/to/project --lang java --skip target,build

# C#
cargo run -- /path/to/project --lang csharp --skip bin,obj
```

### Scripting Languages

```bash
# Ruby
cargo run -- /path/to/project --lang ruby --skip vendor,tmp

# Perl
cargo run -- /path/to/project --lang perl --skip blib,_build

# Shell/Bash
cargo run -- /path/to/project --lang shell --skip tmp
```

### Data Science

```bash
# R
cargo run -- /path/to/project --lang r --skip renv

# Julia
cargo run -- /path/to/project --lang julia --skip "docs/build"

# MATLAB
cargo run -- /path/to/project --lang matlab --skip bin
```

### Other Languages

```bash
# Scala
cargo run -- /path/to/project --lang scala --skip target,"project/target"

# Groovy
cargo run -- /path/to/project --lang groovy --skip target,build

# Dart
cargo run -- /path/to/project --lang dart --skip build,.dart_tool

# Haskell
cargo run -- /path/to/project --lang haskell --skip dist,.stack-work

# Lua
cargo run -- /path/to/project --lang lua --skip bin
```

## Output Format

The output will be in the following format:
```
filename.ext minified_content_all_on_one_line
```

Example:
```
src/main.js function hello(){console.log("Hello, World!");}
lib/utils.js export const add=(a,b)=>a+b;
```

## Supported Languages

1. Python (.py, .pyw)
2. Java (.java)
3. JavaScript (.js, .jsx, .mjs)
4. C++ (.cpp, .hpp, .cc, .hh, .cxx, .hxx)
5. C# (.cs)
6. PHP (.php)
7. Ruby (.rb)
8. Swift (.swift)
9. TypeScript (.ts, .tsx)
10. Kotlin (.kt, .kts)
11. Go (.go)
12. Rust (.rs)
13. R (.r, .R)
14. MATLAB (.m)
15. VB.NET (.vb)
16. Scala (.scala)
17. Perl (.pl, .pm)
18. Dart (.dart)
19. Objective-C (.m, .mm)
20. Groovy (.groovy, .gvy, .gy, .gsh)
21. Julia (.jl)
22. Haskell (.hs, .lhs)
23. Shell/Bash (.sh, .bash)
24. Lua (.lua)
25. C (.c, .h)
