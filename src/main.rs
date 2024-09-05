// #![warn(clippy::pedantic)]
#![allow(warnings)]

// Module std::io
// Traits, helpers, and type definitions for core I/O functionality.
// The std::io module contains a number of common things you’ll need
// when doing input and output. The most core part of this module is
// the Read and Write traits, which provide the most general interface
// for reading and writing input and output.

// Function std::io::stdin
// Constructs a new handle to the standard input of the current process.

use aa_date_parser::*;
use std::io::stdin;
use crate::date_formatting::select_format;
pub mod date_formatting;

fn main() {
    let input_handle = stdin();

    println!("{}", non_localised_text::LANGUAGE_SELECTION_PROMPT);
    let language_id = get_preferred_language(&input_handle).get_id();

    println!("{}", localised_text::INPUT_DATE_PROMPT[language_id]);
    let (weekday, day, month, year) = get_date(&input_handle);

    println!(
        "{}",
        localised_text::OUTPUT_FORMAT_SELECTION_PROMPT[language_id]
    );
    let selected_format = select_format(input_handle);

    
    // TODO: we still have to show month and weekday in a correct format and language! and we'd like to use default formatter instead of debug one.
    //println!("{formatted_date}");
}
