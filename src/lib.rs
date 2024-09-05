#![allow(warnings)]
mod date_enums;
pub mod localised_text;
pub mod non_localised_text;

use date_enums::*;
use localised_text::*;
use non_localised_text::DEFAULT_INPUT_FORMAT;

use std::io::Stdin;

pub fn get_date(input_handle: &Stdin) -> (Weekday, i32, Month, i32) {
    // get date from keyboard - format: W:DD:MM:YYYY.
    let mut character_buffer = String::new();
    input_handle.read_line(&mut character_buffer);

    // split date into chunks, divisor is ":", and collect it to Vec<&str>.
    let split_date: Vec<&str> = character_buffer.trim().split(":").collect();
    // if length of split_date is different than 4 - the user fucked up.
    if split_date.len() != 4 {
        panic!(
            "Incorrect format. Required format: {} ",
            DEFAULT_INPUT_FORMAT
        );
    }

    // accesses the 1st element of the vector split_date which is passed to
    // parse function as an argument, then parse function
    // converts the string slice to a corresponding variant of the Weekday enum
    // result of the parse function is bound to a variable weekday
    let weekday = Weekday::parse(split_date[0]);
    let month = Month::parse(split_date[2]);

    let day: i32 = split_date[1].parse().expect("Invalid day");
    if !(1 <= day && day <= 31) {
        panic!("Incorrect day number: {}", day);
    }

    let year: i32 = split_date[3].parse().expect("Invalid year");
    if !(1900 <= year && year <= 2100) {
        panic!("Incorrect year number: {}", year);
    }

    (weekday, day, month, year)
}

// this function takes a reference to Stdin (standard input stream) handle
// and declares Language enum as the return value
pub fn get_preferred_language(input_handle: &Stdin) -> Language {
    // a new String named 'character_buffer' is initialised to hold the user input
    let mut character_buffer = String::new();

    // this function reads a line of input from the standard input stream (input_handle)
    // and stores it in 'character_buffer', read_line function appends
    // a newline character to the buffer
    input_handle.read_line(&mut character_buffer);

    // trim() method removes any leading and trailing whitespace (including the newline character)
    // to_lowercase() method then converts the input String to lowercase, which is then assigned
    // to a String type variable 'selected_language'
    let selected_language = character_buffer.trim().to_lowercase();

    // declares a variable named 'language' of type Language
    let language: Language;

    // as_str() method is called on the String 'selected_language' which returns a string slice
    // the string slice is compared to values "en" and "pl"
    if selected_language.as_str() == "en" {
        // for the 'en' input, assigns the English variant of the Language enum to the 'language' variable.
        language = Language::English;
    } else if selected_language.as_str() == "pl" {
        // for the 'pl' input, assigns the Polish variant of the Language enum to the 'language' variable.
        language = Language::Polish;
    } else {
        // if the input does not match either "en" or "pl", the function panics
        // and the program will terminate
        panic!("Invalid language selection: {}", selected_language);
    }
    // the function returns the determined Language enum variant.
    language
}
