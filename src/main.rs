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

fn main() {
    let input_handle = stdin();

    // 0) Get user's preferred language
    println!("{}", aa_date_parser::LANGUAGE_SELECTION_PROMPT);
    let language = get_preferred_language(&input_handle);

    // 1) ensure the text is correctly localised

    // 2) Get date in format of W:DD:MM:YYYY
    println!("{}", localised_text::INPUT_DATE_PROMPT);
    // fn get_date() -> DANE {...}
    // dane <- get_date(&input_handle);

    let obtained_date = get_date(&input_handle);

    // 4) Asks in what format the user wants to see the output
    println!("{}", localised_text::OUTPUT_FORMAT_SELECTION_PROMPT);

    let mut buffered_data = String::new();
    buffered_data.clear();
    input_handle.read_line(&mut buffered_data);

    let selected_output_format = buffered_data.trim().to_lowercase();

    // 5) output in correct format
    if selected_output_format == "us" {
        println!("{:?}, {:?} {}, {}", obtained_date.0, obtained_date.2, obtained_date.1, obtained_date.3);
    } else if selected_output_format == "en" {
        println!("{:?}, {} {:?} {}", obtained_date.0, obtained_date.1, obtained_date.2, obtained_date.3);
    } else {
        panic!("Selected output format does not exsist.");
    }
}
