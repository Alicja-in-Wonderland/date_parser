pub const INPUT_DATE_PROMPT: [&str; 2] = [
    "Give a date in numerical format W:DD:MM:YYYY (where: W - weekday, DD - day, MM - month, YYYY - year):",
    "Podaj datę w formacie liczbowym: D:DD:MM:RRRR (gdzie: D - dzień tygodnia, DD - dzień miesiąca, MM - miesiąc, RRRR - rok)"];

pub const OUTPUT_FORMAT_SELECTION_PROMPT: [&str; 2] = [
    "Choose output format: US or EU",
    "Wybiesz format daty: US (amerykański) or EU (europejski)",
];


pub enum Language {
    English,
    Polish,
}
impl Language {
    pub fn get_id(&self) -> usize {
        match self {
            Language::English => 0,
            Language::Polish => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_usize_ok() {
        const TEXTS: [&str; 2] = ["English", "Polish"];

        assert_eq!(0 as i32, Language::English as i32);
        assert_eq!(1 as usize, Language::Polish as usize);
        assert_eq!(TEXTS[0], TEXTS[Language::English.get_id()]);

        let language = Language::Polish;
        assert_eq!(TEXTS[1], TEXTS[language.get_id()]);
    }
}
