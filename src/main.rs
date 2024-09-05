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

    println!("{}", localised_text::LANGUAGE_SELECTION_PROMPT);
    let language_id = get_preferred_language(&input_handle).get_id();

    println!("{}", localised_text::INPUT_DATE_PROMPT[language_id]);
    let (weekday, day, month, year) = get_date(&input_handle);

    println!(
        "{}",
        localised_text::OUTPUT_FORMAT_SELECTION_PROMPT[language_id]
    );

    // this is data type with some implementation, should be elswhere
    enum OutputFormat {
        US,
        EU,
    }
    impl OutputFormat {
        fn get_id(&self) -> usize {
            match self {
                OutputFormat::US => 0,
                OutputFormat::EU => 1,
            }
        }
    }

    // abstract this out to a function:
    let selected_format = {
        let mut buffered_data = String::new();
        buffered_data.clear();
        input_handle.read_line(&mut buffered_data);

        match buffered_data.trim().to_lowercase().as_str() {
            "us" => OutputFormat::US,
            "eu" => OutputFormat::EU,
            _ => {
                panic!("Selected output format does not exsist.")
            }
        }
    };

    //a u can also abstract this out to a function
    let formatted_date = {
        match selected_format {
            OutputFormat::US => format!("{0:?}, {2:?} {1}, {3}", weekday, day, month, year),
            OutputFormat::EU => format!("{0:?}, {1} {2:?} {3}", weekday, day, month, year),
        }
    };

    // TODO: we still have to show month and weekday in a correct format and language! and we'd like to use default formatter instead of debug one.
    println!("{formatted_date}");
}
