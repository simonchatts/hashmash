//! Take all the options and make a `transformer` from any `BufRead` to `Write`.

use regex::{Captures, Regex};
use std::io::{self, BufRead, IsTerminal, Write, stdout};

use crate::classify;
use crate::opts::Opts;
use crate::randomize::randomize;

/// Encapsulate all the one-time processing into a set of data enabling
/// reasonably efficient per-file processing.
pub struct Transformer {
    pre_classifier: Regex,
    transform_hash: fn(&str) -> String,
    transform_non_hash: fn(&str) -> String,
}

impl Transformer {
    /// Do all the one-time processing for the command-line arguments, and save
    /// the state ready for per-file processing.
    pub fn new(opts: &Opts) -> Self {
        // Pre-classifier to zoom in on candidate strings - this is just a quick
        // search string that includes any hash we want to transform. For
        // example, it doesn't matter that a base64 string might also include
        // characters like `[/+=]`, since we expect there'll be runs matching
        // this within a longer string, and we'll transform enough to hide the
        // underlying secret.
        let pre_classifier = Regex::new(r"([a-zA-Z0-9-]{8,})").unwrap();

        // Whether or not stdout is a tty
        let use_colour = !opts.in_place && stdout().is_terminal();

        // Select the appropriate transform functions for the options
        let transform_hash = match (opts.replace, use_colour) {
            (true, true) => randomize_and_highlight_in_red,
            (true, false) => randomize,
            (false, true) => highlight_in_red,
            (false, false) => identity,
        };
        let transform_non_hash = if opts.debug && use_colour {
            highlight_in_green
        } else {
            identity
        };

        // Save all this state.
        Transformer {
            pre_classifier,
            transform_hash,
            transform_non_hash,
        }
    }

    /// Process a file using the pre-computed options.
    pub fn run<R, W>(&mut self, reader: R, writer: &mut W) -> io::Result<()>
    where
        R: BufRead,
        W: Write,
    {
        // For each line, first use the regex pre-classifier to zoom in to plausible
        // candidates. Within those, use the trigram classifier to process each as
        // either a hash or non-hash.
        let mut reader = reader;
        let mut line = String::new();
        while reader.read_line(&mut line)? > 0 {
            let new_line =
                self.pre_classifier
                    .replace_all(&line, |captures: &Captures<'_>| {
                        let s = captures.get(1).expect("regex match 1").as_str();
                        if classify::is_hash(s) {
                            (self.transform_hash)(s)
                        } else {
                            (self.transform_non_hash)(s)
                        }
                    });
            writer.write_all(new_line.as_bytes())?;
            line.clear();
        }
        Ok(())
    }
}

//////////////////////////////////////////////////////////////////////////////
//
// Bunch of transformer functions from which to choose. `randomize` also
// fits the type and is another option.

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
static NORMAL: &str = "\x1b[0m";
static RED: &str = "\x1b[31m";
static GREEN: &str = "\x1b[32m";
