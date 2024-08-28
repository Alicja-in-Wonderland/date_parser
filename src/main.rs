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
    // zapytaj użytkownika o język interfejsu, bazując na wyborze
    //  będziemy (później) wyświetlać informacje w odpowiednim języku
    println!("Choose interface language: EN / PL");
    input_handle.read_line(&mut buffered_data);

    // notatka:
    //      jeśli użytkownik nie wpisze EN ani PL
    //      to uzyska stosowną informację

    if (buffered_data.trim() != "EN") && (buffered_data.trim() != "PL") {
        println!("WRONG ANSWER ASSHOLE.");
        return;
    }

    // 1)
    // poproś użytkownika żeby podał datę w jakimś formacie. np. W:DD:MM:YYYY,
    // gdzie W - weekday, DD - day, MM - month, YYYY - year
    println!("Give a date in numerical format W:DD:MM:YYYY (where: W - weekday, DD - day, MM - month, YYYY - year):");

    buffered_data.clear();
    input_handle.read_line(&mut buffered_data);

    // 2)
    // sprawdź czy dane są w poprawnym formacie
    // wypisz to, co użytkownik podał
    let split_date: Vec<&str> = buffered_data.trim().split(":").collect();

    // let trimmed_data: &str =
    //   trim(buffered_data); // shorthand: buffered_data.trim() -> &str
    // let split_iterator: Iter =
    //   split(trimmed_data, ":"); // shorthand: trimmed_data.split(":") -> Iter
    // let split_date: Vec<&str> =
    //   collect(split_iterator); // shorthand: split_iterator.collect() -> Vec<&str>

    println!("{:?}", split_date);
    let weekday = split_date[0].to_lowercase();

    println!("{weekday}");

    let weekdays = [
        "monday",
        "tuesday",
        "wednesday",
        "thursday",
        "friday",
        "saturday",
        "sunday",
    ];
    let contains = weekdays.contains(&weekday.as_str());

    // 3)
    // zadecyduj o odpowiednich zmiennych w których przechowamy ODDZIELNIE dane
    // weekday, day, month, year
    let weekday: u8 = split_date[0].parse().unwrap();
    let day: u8 = split_date[1].parse().unwrap();
    let month: u8 = split_date[2].parse().unwrap();
    let year: u16 = split_date[3].parse().unwrap();

    // let (weekday, day, month, year): (u8, u8, u8, u16) = (split_date[0].parse().unwrap(), split_date[1].parse().unwrap(), split_date[2].parse().unwrap(), split_date[3].parse().unwrap())

    // 4)
    // wyodrębnij te dane z tego, co użytkownik podał

    // 5) zapytaj w jakim języku i w jakim formacie użytkownik chce zobaczyć output

    // 6) wypisz przygotowaną informację
}
