use std::fmt::Display;

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

impl Display for Weekday {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "dupa")
    }
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
