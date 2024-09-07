use std::io::Stdin;

use crate::date;

pub const DEFAULT: &str = "W:DD:MM:YYYY";

pub enum OutputFormat {
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

pub fn select_format(input_handle: Stdin) -> OutputFormat {
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
}

pub fn format_date(
    language: crate::Language,
    format: OutputFormat,
    weekday: date::Weekday,
    day: i32,
    month: date::Month,
    year: i32,
) -> String {
    let month = crate::strings::localized::MONTHS[language as usize][month as usize];
    let weekday = crate::strings::localized::WEEKDAYS[language as usize][weekday as usize];
    match format {
        OutputFormat::US => format!("{0}, {2} {1}, {3}", weekday, day, month, year),
        OutputFormat::EU => format!("{0}, {1} {2} {3}", weekday, day, month, year),
    }
}