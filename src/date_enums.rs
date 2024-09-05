#[derive(Debug)]
pub enum Weekday {
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
    Sun,
}
// add implementation for the enum Weekday
// implementation = functions/methods
impl Weekday {
    // function parse(some_string) -> Weekday
    // Weekday::parse("1") -> Weekday::Monday
    // Weekday::parse("7") -> Weekday::Sun
    pub fn parse(string: &str) -> Self {
        // let weekday_num: i32 = string.parse().expect("Invalid weekday");

        // // let weekday = match weekday_num {
        // //     1 => Weekday::Mon,
        // //     2 => Weekday::Tue,
        // //     3 => Weekday::Wed,
        // //     4 => Weekday::Thu,
        // //     5 => Weekday::Fri,
        // //     6 => Weekday::Sat,
        // //     7 => Weekday::Sun,
        // //     _ => panic!("Invalid weekday number: {}", weekday_num),
        // // };

        // let weekday;

        // if weekday_num == 1 {
        //     weekday = Weekday::Mon;
        // } else if weekday_num == 2 {
        //     weekday = Weekday::Tue;
        // } else if weekday_num == 3 {
        //     weekday = Weekday::Wed;
        // } else if weekday_num == 4 {
        //     weekday = Weekday::Thu;
        // } else if weekday_num == 5 {
        //     weekday = Weekday::Fri;
        // } else if weekday_num == 6 {
        //     weekday = Weekday::Sat;
        // } else if weekday_num == 7 {
        //     weekday = Weekday::Sun;
        // } else {
        //     panic!("Invalid weekday number: {}", weekday_num);
        // }

        // weekday
        match string.parse().unwrap() {
            1 => Weekday::Mon,
            2 => Weekday::Tue,
            3 => Weekday::Wed,
            4 => Weekday::Thu,
            5 => Weekday::Fri,
            6 => Weekday::Sat,
            7 => Weekday::Sun,
            _ => panic!("Invalid weekday"),
        }
    }
}

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
