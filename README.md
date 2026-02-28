# grep_clone

`grep_clone` is a fast, multithreaded, Rust-based search CLI inspired by `grep`.
It recursively scans files in a target directory, matches lines with a regex pattern, and prints colored results with file paths and line numbers.

## Features

- Regex-powered search using `regex`
- Recursive directory traversal
- Parallel file processing using all CPU cores by default
- Case-insensitive matching with `-i`
- Match counting per file with `-c`
- Custom thread count via `--threads N`
- Respects `.gitignore` and `.git/info/exclude`
- Basic binary-file protection by skipping lines containing null bytes
- Search duration summary at the end

## Usage

```bash
cargo run -- [OPTIONS] <pattern> <directory>
```

### Options

- `-i` : case-insensitive search
- `-c` : show only match count per file
- `--threads N` : set number of worker threads (default: number of CPU cores)

## Examples

```bash
# Search for "main" in src directory
cargo run -- "main" src

# Case-insensitive search in current directory
cargo run -- -i "todo" .

# Count matches per file using 8 threads
cargo run -- -c --threads 8 "error|warn" .
```

## Output Format

- Normal mode: `<file_path>:<line_number>: <line_with_colored_match>`
- Count mode: `<file_path>: <match_count>`

At the end of every run:

```text
Search completed in <ms> ms using <threads> threads
```

## Tech Stack

- Rust (Edition 2024)
- `regex`
- `rayon`
- `ignore`
- `colored`
- `num_cpus`
