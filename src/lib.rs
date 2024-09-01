#![allow(warnings)]
mod date_enums;
pub mod localised_text;

pub const LANGUAGE_SELECTION_PROMPT: &str =
    "Choose interface language / Wybierz język interfejsu: EN / PL";
// pub const LANGUAGE_ERROR: &str = "Language selection string not matching any pattern.";
pub const INPUT_ERROR: &str = "Invalid input.";

use date_enums::*;
use std::io::Stdin;
pub enum Language {
    English,
    Polish,
}

pub fn get_date(input_handle: &Stdin) -> Result<(), ()> {
    // get date from keyboard
    // format: W:DD:MM:YYYY
    let mut character_buffer = String::new();
    input_handle.read_line(&mut character_buffer);

    // 2)
    // Checks if user input is in the correct format and prints it
    let split_date: Vec<&str> = character_buffer.trim().split(":").collect();
    if split_date.len() != 4 {
        return Err(());
    }

    let weekday = match split_date[0].parse().expect(INPUT_ERROR) {
        1 => Weekday::Mon,
        2 => Weekday::Tue,
        3 => Weekday::Wed,
        4 => Weekday::Thu,
        5 => Weekday::Fri,
        6 => Weekday::Sat,
        7 => Weekday::Sun,
        _ => return Err(()),
    };

    let day: &str = split_date[1];

    let month = Month::parse(split_date[2]);

    let year: u16 = split_date[3].parse().unwrap();
    let years: Vec<_> = (1900..=2030).collect();
    let is_year_correct: bool = years.contains(&year);
    if !is_year_correct {
        println!("FIXME");
    }

    let day: u8 = day.parse().expect("FIXME");
    let days: Vec<_> = (1..=31).collect();
    let is_day_correct = days.contains(&day);
    if !is_day_correct {
        println!("FIXME");
    }
    // 4)
    // Extracts data from user's input
    Ok(())
}

// if selected_language == "en" {
//     Ok(Language::English)
// } else if selected_language == "pl" {
//     Ok(Language::Polish)
// } else {
//     Err(())
// }

pub fn get_preferred_language(input_handle: &Stdin) -> Result<Language, ()> {
    let mut character_buffer = String::new();

    input_handle.read_line(&mut character_buffer);
    let selected_language = character_buffer.trim().to_lowercase();

    match selected_language.as_str() {
        "en" => Ok(Language::English),
        "pl" => Ok(Language::Polish),
        _ => Err(()),
    }
}
