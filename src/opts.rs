//! Type defining the command-line arguments (help text comes from docstrings).
use clap::Parser;

/// Highlight or randomize strings that look like cryptographic hashes or GUIDs
#[derive(Parser)]
#[clap(version = "1.0.1")] // Keep in sync with flake.nix and Cargo.toml
pub struct Opts {
    /// Actually randomize any identified hashes, don't just highlight them
    #[clap(short, long)]
    pub replace: bool,

    /// Destructively modify files in-place (implies --replace)
    #[clap(short, long)]
    pub in_place: bool,

    /// Highlight strings in green that are not classified as hashes, but are
    /// included for consideration
    #[clap(short, long)]
    pub debug: bool,

    /// File(s) to process, or standard input if omited
    pub input_file: Vec<String>,
}
