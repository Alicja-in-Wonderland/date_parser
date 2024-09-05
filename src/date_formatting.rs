use std::io::Stdin;
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

/* // abstract this out to a function:
    let selected_format = {
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
    }; */

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

    /* //a u can also abstract this out to a function
    let formatted_date = {
        match selected_format {
            OutputFormat::US => format!("{0:?}, {2:?} {1}, {3}", weekday, day, month, year),
            OutputFormat::EU => format!("{0:?}, {1} {2:?} {3}", weekday, day, month, year),
        }
    };
    */
    
    pub fn format_date(select_format: OutputFormat, weekday: &str, day: i32, month: &str, year: i32) -> String {
        let formatted_date = match select_format {
            OutputFormat::US => format!("{0}, {2} {1}, {3}", weekday, day, month, year),
            OutputFormat::EU => format!("{0}, {1} {2} {3}", weekday, day, month, year),
        };

        return formatted_date;
    }