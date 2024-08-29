#![allow(warnings)]

// Module std::io
// Traits, helpers, and type definitions for core I/O functionality.
// The std::io module contains a number of common things you’ll need
// when doing input and output. The most core part of this module is
// the Read and Write traits, which provide the most general interface
// for reading and writing input and output.

// Function std::io::stdin
// Constructs a new handle to the standard input of the current process.

use std::{io::stdin, path::Iter};

fn main() {
    let input_handle = stdin();
    let mut buffered_data = String::new();

    // 0)
    // Asks user to choose interface language between English and Polish,
    //  depending on the selection, info will be displayed in the chosen language
    println!("Choose interface language / Wybierz język interfejsu: EN / PL");
    input_handle.read_line(&mut buffered_data);

    // Note:
    //      If user doesn't type EN nor PL
    //      they will receive a notification

    if (buffered_data.trim() != "EN") && (buffered_data.trim() != "PL") {
        println!("WRONG ANSWER. Bye");
        return;
    }

    if buffered_data.trim() == "EN" {

        // 1)
        // Asks user to type a date in a given numerical format: W:DD:MM:YYYY,
        // where W - weekday, DD - day, MM - month, YYYY - year
        println!("Give a date in numerical format W:DD:MM:YYYY (where: W - weekday, DD - day, MM - month, YYYY - year):");

        buffered_data.clear();
        input_handle.read_line(&mut buffered_data);

        // 2)
        // Checks if user input is in the correct format and prints it
        let split_date: Vec<&str> = buffered_data.trim().split(":").collect();

        // let trimmed_data: &str =
        //   trim(buffered_data); // shorthand: buffered_data.trim() -> &str
        // let split_iterator: Iter =
        //   split(trimmed_data, ":"); // shorthand: trimmed_data.split(":") -> Iter
        // let split_date: Vec<&str> =
        //   collect(split_iterator); // shorthand: split_iterator.collect() -> Vec<&str>

        println!("User input after split: {:?}", split_date);
        let weekday = split_date[0];
        let day: &str = split_date[1];
        let month = split_date[2];
        let year: u16 = split_date[3].parse().unwrap();

        let weekdays = ["1", "2", "3", "4", "5", "6", "7"];

        let days = [
            "01", "02", "03", "04", "05", "06", "07", "08", "09", "10", "11", "12", "13", "14",
            "15", "16", "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27", "28",
            "29", "30", "31",
        ];

        let months = [
            "01", "02", "03", "04", "05", "06", "07", "08", "09", "10", "11", "12",
        ];

        let years = [1900..2030];

        let contains_weekday = weekdays.contains(&weekday);

        if !contains_weekday {
            println!("Weekday in wrong format or missing!");
        }

        let contains_day: bool = days.contains(&day);

        if !contains_day {
            println!("Day in wrong format or missing!");
        }

        let contains_month: bool = months.contains(&month);

        if !contains_month {
            println!("Month in wrong format or missing!");
        }



        // 3)
        // Declaring variables to store data in: weekday, day, month and year

        let weekday: u8 = split_date[0].parse().unwrap();
        let day: u8 = split_date[1].parse().unwrap();
        let month: u8 = split_date[2].parse().unwrap();
        let year: u16 = split_date[3].parse().unwrap();

        // let (weekday, day, month, year): (u8, u8, u8, u16) = (split_date[0].parse().unwrap(), split_date[1].parse().unwrap(), split_date[2].parse().unwrap(), split_date[3].parse().unwrap())

        // 4)
        // Extracts data from user's input

        // 5) Asks in what language and format the user wants to see the output

        // 6) Prints prepared information
    } else {
        println!("Polish version in the making...Stay tuned! / Wersja polska w przygotowaniu...");
    }
}
