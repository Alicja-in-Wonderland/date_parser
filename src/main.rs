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

mod date;
mod strings;
mod user_input;

use std::io::stdin;

use crate::date::format::{format_date, select_format, OutputFormat};

fn main() {
    let input_handle = stdin();

    println!("{}", strings::non_localized::LANGUAGE_SELECTION_PROMPT);
    let language = user_input::get_preferred_language(&input_handle);

    println!(
        "{}",
        strings::localized::INPUT_DATE_PROMPT[language as usize]
    );
    let date = user_input::get_date(&input_handle);

    println!(
        "{}",
        strings::localized::OUTPUT_FORMAT_SELECTION_PROMPT[language as usize]
    );
    let format = select_format(input_handle);

    let formatted_date = format_date(
        language,
        format,
        date.weekday,
        date.day,
        date.month,
        date.year,
    );

    println!("{}", formatted_date);
}

#[derive(Clone, Copy)]
pub enum Language {
    English,
    Polish,
}
// impl Language {
//     pub fn get_id(&self) -> usize {
//         match self {
//             Language::English => 0,
//             Language::Polish => 1,
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_usize_ok() {
        const TEXTS: [&str; 2] = ["English", "Polish"];

        assert_eq!(0 as i32, Language::English as i32);
        assert_eq!(1 as usize, Language::Polish as usize);
        // assert_eq!(TEXTS[0], TEXTS[Language::English.get_id()]);
        assert_eq!(TEXTS[0], TEXTS[Language::English as usize]);

        let language = Language::Polish;
        // assert_eq!(TEXTS[1], TEXTS[language.get_id()]);
        assert_eq!(TEXTS[1], TEXTS[language as usize]);
    }
}