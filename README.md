# rust-spitter

A universal code minifier that supports multiple programming languages. This tool finds all relevant source files in a directory, minifies them, and outputs them in a standardized format.

Originally built to create a contiguous block of code from a project that could be used in graphical and art applications. **PRs are welcome**.

## Basic Usage

```bash
cargo run -- <directory> --lang <language> [--skip dir1,dir2,dir3]
```

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
cmd/dependencycheck/dependencycheck.go /Copyright2020TheKubernetesAuthors.LicensedundertheApacheLicense,Version2.0(the"License");youmaynotusethisfileexceptincompliancewiththeLicense.YoumayobtainacopyoftheLicenseathttp:Unlessrequiredbyapplicablelaworagreedtoinwriting,softwaredistributedundertheLicenseisdistributedonan"AS IS"BASIS,WITHOUTWARRANTIESORCONDITIONSOFANYKIND,eitherexpressorimplied.SeetheLicenseforthespecificlanguagegoverningpermissionsandlimitationsundertheLicense.*/packagemainimport("bytes""encoding/json""flag""fmt""io""log""os""regexp")var(exclude=flag.String("exclude","","skip packages regex pattern (e.g. '^k8s.io/kubernetes/')")restrict=flag.String("restrict","","restricted dependencies regex pattern (e.g. '^k8s.io/(apimachinery|client-go)/')"))typegoPackagestruct{NamestringImportPathstringImports[]stringTestImports[]stringXTestImports[]string}funcmain(){flag.Parse()args:=flag.Args()iflen(args)!=1{log.Fatalf("usage: dependencycheck <json-dep-file> (e.g. 'go list -mod=vendor -test -deps -json ./vendor/...')")}if*restrict==""{log.Fatalf("Must specify restricted regex pattern")}depsPattern,err:=regexp.Compile(*restrict)iferr!=nil{log.Fatalf("Error compiling restricted dependencies regex: %v",err)}varexcludePattern*regexp.Regexpif*exclude!=""{excludePattern,err=regexp.Compile(*exclude)iferr!=nil{log.Fatalf("Error compiling excluded package regex: %v",err)}}b,err:=os.ReadFile(args[0])iferr!=nil{log.Fatalf("Error reading dependencies file: %v",err)}packages:=[]goPackage{}decoder:=json.NewDecoder(bytes.NewBuffer(b))for{pkg:=goPackage{}iferr:=decoder.Decode(&pkg);err!=nil{iferr==io.EOF{break}log.Fatalf("Error unmarshaling dependencies file: %v",err)}packages=append(packages,pkg)}violations:=map[string][]string{}for_,p:=rangepackages{ifexcludePattern!=nil&&excludePattern.MatchString(p.ImportPath){continue}importViolations:=[]string{}allImports:=[]string{}allImports=append(allImports,p.Imports...)allImports=append(allImports,p.TestImports...)allImports=append(allImports,p.XTestImports...)for_,i:=rangeallImports{ifdepsPattern.MatchString(i){importViolations=append(importViolations,i)}}iflen(importViolations)>0{violations[p.ImportPath]=importViolations}}iflen(violations)>0{fork,v:=rangeviolations{fmt.Printf("Found dependency violations in package %s:\n", k)\n for _, a := range v {\n fmt.Println("-->" + a)\n }\n }\n log.Fatal("Foundrestricteddependencyviolationsinpackages")\n }\n}\n
cmd/genyaml/gen_kubectl_yaml.go /Copyright2014TheKubernetesAuthors.LicensedundertheApacheLicense,Version2.0(the"License");youmaynotusethisfileexceptincompliancewiththeLicense.YoumayobtainacopyoftheLicenseathttp:Unlessrequiredbyapplicablelaworagreedtoinwriting,softwaredistributedundertheLicenseisdistributedonan"AS IS"BASIS,WITHOUTWARRANTIESORCONDITIONSOFANYKIND,eitherexpressorimplied.SeetheLicenseforthespecificlanguagegoverningpermissionsandlimitationsundertheLicense.*/packagemainimport("bytes""fmt""io""os""strings""github.com/spf13/cobra""github.com/spf13/pflag""k8s.io/cli-runtime/pkg/genericiooptions""k8s.io/kubectl/pkg/cmd""k8s.io/kubernetes/cmd/genutils"yaml"sigs.k8s.io/yaml/goyaml.v2")typecmdOptionstruct{NamestringShorthandstring`yaml:",omitempty"`DefaultValuestring`yaml:"default_value,omitempty"`Usagestring`yaml:",omitempty"`}typecmdDocstruct{NamestringSynopsisstring`yaml:",omitempty"`Descriptionstring`yaml:",omitempty"`Options[]cmdOption`yaml:",omitempty"`InheritedOptions[]cmdOption`yaml:"inherited_options,omitempty"`Examplestring`yaml:",omitempty"`SeeAlso[]string`yaml:"see_also,omitempty"`}funcmain(){path:="docs/yaml/kubectl"iflen(os.Args)==2{path=os.Args[1]}elseiflen(os.Args)>2{fmt.Fprintf(os.Stderr,"usage: %s [output directory]\n", os.Args[0])\n os.Exit(1)\n }\n\n outDir, err := genutils.OutDir(path)\n if err != nil {\n fmt.Fprintf(os.Stderr, "failedtogetoutputdirectory:%v\n", err)\n os.Exit(1)\n }\n\n // Set environment variables used by kubectl so the output is consistent,\n // regardless of where we run.\n os.Setenv("HOME", "/me/ername")\n kubectl := cmd.NewKubectlCommand(cmd.KubectlOptions{IOStreams: genericiooptions.IOStreams{In: bytes.NewReader(nil), Out: io.Discard, ErrOut: io.Discard}})\n genYaml(kubectl, "", outDir)\n for _, c := range kubectl.Commands() {\n genYaml(c, "kubectl", outDir)\n }\n}\n\n// Temporary workaround for yaml lib generating incorrect yaml with long strings\n// that do not contain \n.\nfunc forceMultiLine(s string) string {\n if len(s) > 60 && !strings.Contains(s, "\n") {\n s = s + "\n"\n }\n return s\n}\n\nfunc genFlagResult(flags *pflag.FlagSet) []cmdOption {\n result := []cmdOption{}\n\n flags.VisitAll(func(flag *pflag.Flag) {\n // Todo, when we mark a shorthand is deprecated, but specify an empty message.\n // The flag.ShorthandDeprecated is empty as the shorthand is deprecated.\n // Using len(flag.ShorthandDeprecated) > 0 can't handle this, others are ok.\n if !(len(flag.ShorthandDeprecated) > 0) && len(flag.Shorthand) > 0 {\n opt := cmdOption{\n flag.Name,\n flag.Shorthand,\n flag.DefValue,\n forceMultiLine(flag.Usage),\n }\n result = append(result, opt)\n } else {\n opt := cmdOption{\n Name: flag.Name,\n DefaultValue: forceMultiLine(flag.DefValue),\n Usage: forceMultiLine(flag.Usage),\n }\n result = append(result, opt)\n }\n })\n\n return result\n}\n\nfunc genYaml(command *cobra.Command, parent, docsDir string) {\n doc := cmdDoc{}\n\n doc.Name = command.Name()\n doc.Synopsis = forceMultiLine(command.Short)\n doc.Description = forceMultiLine(command.Long)\n\n flags := command.NonInhe
...
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
