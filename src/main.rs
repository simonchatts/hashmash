#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

//! Identify substrings that look like cryptographic hashes/GUIDs/UUIDs/etc
//! in text files, and optionally randomize them (for, eg, documentation).

use clap::Clap;
use std::env;
use std::fs::{self, File};
use std::io::{self, BufReader, BufWriter};
use std::process;

mod classify;
mod opts;
mod randomize;
mod transform;
use opts::Opts;
use transform::Transformer;

/// App entrypoint. Just some basic processing, then jump to a `Result` context.
pub fn main() {
    let mut opts = Opts::parse();
    let prog_name = env::args().next().unwrap();

    // Enforce configuration option invariants: --in-place implies --replace, and
    // also requires actual files rather than stdin.
    if opts.in_place {
        if opts.input_file.is_empty() {
            eprintln!(
                "{}: the --in-place flag requires explicit input files, not just stdin",
                prog_name
            );
            process::exit(1);
        }
        opts.replace = true;
    }

    // Do the rest in a Result context.
    process(opts).unwrap_or_else(|err| eprintln!("{}: Error: {}", prog_name, err))
}

/// Bulk of the app logic.
fn process(opts: Opts) -> io::Result<()> {
    // Get an output buffer for stdout.
    let stdout_handle = io::stdout();
    let mut stdout = stdout_handle.lock();
    // Get a transformer object to transform inputs to outputs based on opts.
    let mut transformer = Transformer::new(&opts);

    if opts.input_file.is_empty() {
        //
        // No filenames, so just stdin -> stdout.
        //
        let stdin_handle = io::stdin();
        let stdin = stdin_handle.lock();
        transformer.run(stdin, &mut stdout)?;
    } else {
        //
        // Specified filenames.
        //
        for filename in &opts.input_file {
            let file = File::open(filename)?;
            let input_file = BufReader::new(file);
            if !opts.in_place {
                //
                // Not transform-in-place, so just send output to stdout.
                //
                transformer.run(input_file, &mut stdout)?;
            } else {
                //
                // Transform in-place. Write to a temporary file, then atomically rename
                // that back to the original filename. Could be smarter about failures.
                //

                // Cheesy unlikely-to-exist temporary file, on the same
                // filesystem as the target. We do it this way (vs eg `mkstemp`)
                // as a cheap way to achieve Windows compatibility.
                let temp_filename = format!("{}~~~hashmash-{}", filename, process::id());
                {
                    // Create a scope just for fastidiousness (not required) so the
                    // temporary file is closed before we rename it.
                    let output_file = File::create(&temp_filename)?;
                    let mut output_file = BufWriter::new(output_file);
                    transformer.run(input_file, &mut output_file)?;
                }
                fs::rename(temp_filename, filename)?;
                eprintln!("Edited file {}", filename);
            }
        }
    }
    Ok(())
}
