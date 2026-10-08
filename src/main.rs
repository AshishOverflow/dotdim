mod compare;
mod mask;
mod parser;

use clap::Parser;
use colored::Colorize;
use is_terminal::IsTerminal;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;
use std::process;

use compare::{extract_keys, find_missing_keys};
use parser::{parse_line, render_line, LintError, ParsedLine};

#[derive(Parser, Debug)]
#[command(
    name = "dotdim",
    version,
    about = "A lightning-fast CLI utility that colorizes and intelligently masks your .env files",
    long_about = "dotdim parses .env files, formats key-value pairs with syntax highlighting, \
and masks sensitive secrets so you can safely inspect and share environment configurations."
)]
struct Args {
    /// Path to the .env file (defaults to .env, or stdin if piped)
    #[arg(value_name = "FILE")]
    file: Option<PathBuf>,

    /// Compare against a template file and warn on missing keys
    #[arg(short = 'c', long = "compare", value_name = "TEMPLATE")]
    compare: Option<PathBuf>,

    /// Validation mode: exit with non-zero status code if malformed lines are detected
    #[arg(short = 'l', long = "lint")]
    lint: bool,

    /// Disable colored ANSI output
    #[arg(long = "no-color")]
    no_color: bool,
}

fn main() {
    let args = Args::parse();

    // Color Fallback:
    // Automatically strip ANSI codes if outputting to a non-TTY environment (e.g. pipe to file)
    // or if the NO_COLOR environment variable is set, or if explicitly requested via --no-color.
    let is_stdout_tty = io::stdout().is_terminal();
    let no_color_env = std::env::var_os("NO_COLOR").is_some();
    if !is_stdout_tty || no_color_env || args.no_color {
        colored::control::set_override(false);
    }

    // Determine input source:
    // 1. If explicit file path is provided:
    //    - If "-" -> read from stdin
    //    - Else read from file path
    // 2. If no file path is provided:
    //    - If stdin is piped (!is_terminal), read from stdin
    //    - Else default to ".env"
    let is_stdin_pipe = !io::stdin().is_terminal();
    let target_file = match &args.file {
        Some(f) if f.to_string_lossy() == "-" => None,
        Some(f) => Some(f.clone()),
        None => {
            if is_stdin_pipe {
                None // read from stdin
            } else {
                Some(PathBuf::from(".env"))
            }
        }
    };

    // Open reader
    let reader: Box<dyn BufRead> = match target_file {
        Some(ref path) => match File::open(path) {
            Ok(file) => Box::new(BufReader::new(file)),
            Err(e) => {
                eprintln!(
                    "{}",
                    format!("Error: Failed to open file '{}': {}", path.display(), e)
                        .bold()
                        .red()
                );
                process::exit(1);
            }
        },
        None => Box::new(BufReader::new(io::stdin())),
    };

    let mut local_keys = HashSet::new();
    let mut lint_errors: Vec<LintError> = Vec::new();

    // Process line-by-line using BufReader for streaming memory efficiency
    for (idx, line_res) in reader.lines().enumerate() {
        let line_num = idx + 1;
        let line = match line_res {
            Ok(l) => l,
            Err(e) => {
                eprintln!(
                    "{}",
                    format!("Error: Failed to read line {}: {}", line_num, e)
                        .bold()
                        .red()
                );
                process::exit(1);
            }
        };

        let (parsed, err) = parse_line(&line, line_num);

        if let ParsedLine::KeyValue { ref key, .. } = parsed {
            local_keys.insert(key.clone());
        }

        if let Some(err) = err {
            if args.lint {
                lint_errors.push(err);
            }
        }

        // Output rendered line to stdout
        println!("{}", render_line(&parsed));
    }

    // Template Comparison (Phase 5)
    if let Some(template_path) = &args.compare {
        match File::open(template_path) {
            Ok(file) => {
                let template_reader = BufReader::new(file);
                match extract_keys(template_reader) {
                    Ok(template_keys) => {
                        let missing = find_missing_keys(&template_keys, &local_keys);
                        if !missing.is_empty() {
                            eprintln!();
                            for key in &missing {
                                eprintln!(
                                    "{}",
                                    format!(
                                        "Warning: Missing key '{}' defined in template '{}'",
                                        key,
                                        template_path.display()
                                    )
                                    .bold()
                                    .red()
                                );
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!(
                            "{}",
                            format!(
                                "Error: Failed to read template file '{}': {}",
                                template_path.display(),
                                e
                            )
                            .bold()
                            .red()
                        );
                        process::exit(1);
                    }
                }
            }
            Err(e) => {
                eprintln!(
                    "{}",
                    format!(
                        "Error: Failed to open template file '{}': {}",
                        template_path.display(),
                        e
                    )
                    .bold()
                    .red()
                );
                process::exit(1);
            }
        }
    }

    // Validation Mode (Phase 5)
    if args.lint && !lint_errors.is_empty() {
        eprintln!();
        eprintln!(
            "{}",
            format!(
                "Lint check failed: {} malformed line(s) detected.",
                lint_errors.len()
            )
            .bold()
            .red()
        );
        for err in lint_errors {
            eprintln!(
                "{}",
                format!("  [Line {}] {}: {:?}", err.line_number, err.reason, err.raw)
                    .bold()
                    .red()
            );
        }
        process::exit(1);
    }
}
