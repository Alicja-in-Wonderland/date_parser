use std::fmt::Display;

#[derive(Debug)]
pub enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}
impl Display for Month {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "dupa")
    }
}
// add implementation for the enum Month
impl Month {
    // Ensures only valid Month values are accepted
    // function parse takes a string slice as an argument, declares Self (Month) as return value
    pub fn parse(string: &str) -> Self {
        // it attempts to parse this string slice into an integer i32 using string.parse() and
        // assign parsed integer to variable 'month_num'
        // if the slice cannot be parsed into int, the program will result in the "Invalid month" message
        let month_num: i32 = string.parse().expect("Invalid month");

        // parsed integer 'month_num' is then matched against specific values from 1 to 12.
        // Each value corresponds to a variant of the 'Month' enum
        let month = match month_num {
            1 => Month::January,
            2 => Month::February,
            3 => Month::March,
            4 => Month::April,
            5 => Month::May,
            6 => Month::June,
            7 => Month::July,
            8 => Month::August,
            9 => Month::September,
            10 => Month::October,
            11 => Month::November,
            12 => Month::December,

            // if the 'month_num' does not match any of the cases, the function will panic and
            // print a message "Invalid month:"
            _ => panic!("Invalid month: {}.", month_num),
        };

        // if the month_num matches one of the cases, the function parse returns the 'Month' enum variant
        // that corresponds to the parsed month number
        month
    }
}
