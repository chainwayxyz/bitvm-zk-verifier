// bitvm-transpiler - Convert other assemblies to BitVM assembly.
// Copyright (C) 2024 Chainway Labs
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! This module parses CLI arguments and options.

use clap::Parser;
use file_format::FileFormat;
use std::path::PathBuf;

/// Command line arguments and flags. Clap will handle these.
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Input file to be converted to BitVM assembly.
    input: Option<PathBuf>,
}

/// Main function to parse all command line inputs.
pub fn parse_cli() -> PathBuf {
    let cli = Args::parse();
    let mut input_file = PathBuf::new();

    match cli.input.as_deref() {
        Some(file) => input_file = file.to_owned(),
        None => fatal_error("Input file not given."),
    }

    check_file_format(&input_file);

    input_file
}

/// Checks if file format is supported. Exits program in case of any
/// incompatibility.
fn check_file_format(input_file: &PathBuf) {
    let fmt = FileFormat::from_file(input_file);

    match fmt {
        Ok(FileFormat::ExecutableAndLinkableFormat) => (),
        Err(e) => fatal_error(e.to_string().as_str()),
        _ => fatal_error("Input file format not supported."),
    }
}

/// Prints a simple error message and exits program with an error code in case
/// of any error while parsing CLI arguments.
fn fatal_error(error: &str) {
    panic!("Fatal error while parsing CLI arguments: {}", error);
}
