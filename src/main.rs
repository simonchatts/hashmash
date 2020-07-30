use clap::Clap;
use fileinput::FileInput;
use isatty::stdout_isatty;
use regex::{Captures, Regex};
use std::io::{self, BufRead, BufReader};

mod classify;
mod randomize;
use randomize::randomize;

//////////////////////////////////////////////////////////////////////////////
//
// Type defining the command-line arguments (help text comes from docstrings).
//

/// Highlight or randomize strings that look like cryptographic hashes or GUIDs.
#[derive(Clap)]
#[clap(version = "0.1.0")]
pub struct Opts {
    /// Actually randomize any identified hashes, don't just highlight them.
    #[clap(short, long)]
    replace: bool,

    /// Highlight strings in green that are not classified as hashes, but are
    /// included for consideration.
    #[clap(short, long)]
    debug: bool,

    /// File(s) to process, or standard input if omited.
    input_file: Vec<String>,
}

//////////////////////////////////////////////////////////////////////////////
//
// Application logic
//

/// Slightly nicer wrapper around `Result` than just panicking on error.
pub fn main() {
    let opts: Opts = Opts::parse();
    process(opts).unwrap_or_else(|e| eprintln!("Error: {}", e))
}

/// Main app logic.
fn process(opts: Opts) -> io::Result<()> {
    // Pre-classifier to zoom in on candidate strings
    let pre_classifier = Regex::new(r"([a-zA-Z0-9-]{8,})").unwrap();

    // Select the appropriate transform functions for the options
    let transform_hash = match (opts.replace, stdout_isatty()) {
        (true, true) => randomize_and_highlight_in_red,
        (true, false) => randomize,
        (false, true) => highlight_in_red,
        (false, false) => identity,
    };
    let transform_not_hash = if opts.debug && stdout_isatty() {
        highlight_in_green
    } else {
        identity
    };

    // Do it
    let fileinput = FileInput::new(&opts.input_file);
    let reader = BufReader::new(fileinput);
    for line in reader.lines() {
        let line = line?;
        let new_line = pre_classifier.replace_all(&line, |captures: &Captures| {
            let s = captures.get(1).expect("regex match 1").as_str();
            if classify::is_hash(s) {
                transform_hash(s)
            } else {
                transform_not_hash(s)
            }
        });
        println!("{}", new_line);
    }
    Ok(())
}

//////////////////////////////////////////////////////////////////////////////
//
// Helper bits

// Bunch of string transform functions from which to choose.
// (`randomize` also fits this type.)
fn identity(s: &str) -> String {
    s.to_string()
}

fn highlight_in_green(s: &str) -> String {
    format!("{}{}{}", GREEN, s, NORMAL)
}

fn highlight_in_red(s: &str) -> String {
    format!("{}{}{}", RED, s, NORMAL)
}

fn randomize_and_highlight_in_red(s: &str) -> String {
    format!("{}{}{}", RED, randomize(s), NORMAL)
}

// ANSI colours
static NORMAL: &'static str = "\x1b[30m";
static RED: &'static str = "\x1b[31m";
static GREEN: &'static str = "\x1b[32m";
