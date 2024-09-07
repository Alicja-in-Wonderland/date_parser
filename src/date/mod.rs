#![allow(warnings)]
pub mod format;
mod month;
mod weekday;

pub use month::Month;
pub use weekday::Weekday;

// 1st option - pub struct Date(pub Weekday, pub i32, pub Month, pub i32);

pub struct Date {
    pub weekday: Weekday,
    pub day: i32,
    pub month: Month,
    pub year: i32,
}
