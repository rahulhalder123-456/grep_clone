use std::env;
use std::time::Instant;
use rayon::ThreadPoolBuilder;
use regex::Regex;
use colored::*;
use ignore::WalkBuilder;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let start_time = Instant::now();

    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} [-i] [-c] [--threads N] <pattern> <directory>", args[0]);
        std::process::exit(1);
    }

    let mut case_insensitive = false;
    let mut count_mode = false;
    let mut thread_count = num_cpus::get();

    let mut non_flag_args = Vec::new();
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "-i" => case_insensitive = true,
            "-c" => count_mode = true,
            "--threads" => {
                if i + 1 < args.len() {
                    thread_count = args[i + 1].parse().unwrap_or(thread_count);
                    i += 1;
                }
            }
            _ => non_flag_args.push(&args[i]),
        }
        i += 1;
    }

    if non_flag_args.len() != 2 {
        eprintln!("Usage: {} [-i] [-c] [--threads N] <pattern> <directory>", args[0]);
        std::process::exit(1);
    }

    let pattern = non_flag_args[0];
    let dir = non_flag_args[1];

    // Configure thread pool
    ThreadPoolBuilder::new()
        .num_threads(thread_count)
        .build_global()
        .unwrap();

    let re = if case_insensitive {
        Regex::new(&format!("(?i){}", pattern)).unwrap()
    } else {
        Regex::new(pattern).unwrap()
    };

    WalkBuilder::new(dir)
        .hidden(false)          // include hidden if needed
        .git_ignore(true)       // respect .gitignore
        .git_exclude(true)
        .build_parallel()
        .run(|| {
            let re = re.clone();
            Box::new(move |entry| {
                if let Ok(entry) = entry {
                    let path = entry.path();

                    if path.is_file() {
                        if let Ok(file) = File::open(path) {
                            let reader = BufReader::new(file);
                            let mut file_match_count = 0;

                            for (line_number, line) in reader.lines().enumerate() {
                                if let Ok(line) = line {

                                    if line.contains('\0') {
                                        return ignore::WalkState::Continue;
                                    }

                                    if re.is_match(&line) {
                                        file_match_count += 1;

                                        if !count_mode {
                                            let highlighted = re.replace_all(&line, |caps: &regex::Captures| {
                                                caps[0].red().bold().to_string()
                                            });

                                            println!(
                                                "{}:{}: {}",
                                                path.display().to_string().cyan(),
                                                (line_number + 1).to_string().yellow(),
                                                highlighted
                                            );
                                        }
                                    }
                                }
                            }

                            if count_mode && file_match_count > 0 {
                                println!(
                                    "{}: {}",
                                    path.display().to_string().cyan(),
                                    file_match_count.to_string().green()
                                );
                            }
                        }
                    }
                }
                ignore::WalkState::Continue
            })
        });

    let duration = start_time.elapsed();

    println!(
        "\nSearch completed in {} ms using {} threads",
        duration.as_millis().to_string().green(),
        thread_count.to_string().yellow()
    );
}