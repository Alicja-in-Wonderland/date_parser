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

    // 0) Get preffered language
    println!("{}", aa_date_parser::LANGUAGE_SELECTION_PROMPT);
    let language = get_preferred_language(&input_handle).expect(INPUT_ERROR);

    match language {
        Language::English => {
            get_date(&input_handle);
            // 5) Asks in what format the user wants to see the output
            println!("Choose output language: EN or PL / Wybierz język: EN lub PL");
            let mut buffered_data = String::new();
            buffered_data.clear();
            input_handle.read_line(&mut buffered_data);

            if (buffered_data.trim() != "EN") && (buffered_data.trim() != "PL") {
                println!("WRONG ANSWER. Bye");
                return;
            }

            if buffered_data.trim() == "EN" {
                println!("Choose output format: US-EN or PL-PL");
            }

            if buffered_data.trim() == "PL" {
                println!("Wybierz format daty: US-EN lub PL-PL");
            }

            // 6) Prints prepared information
        }
        _ => println!("FIXME"),
    }
}
