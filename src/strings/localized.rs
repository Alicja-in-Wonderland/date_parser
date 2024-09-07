pub const INPUT_DATE_PROMPT: [&str; 2] = [
    "Give a date in numerical format W:DD:MM:YYYY (where: W - weekday, DD - day, MM - month, YYYY - year):",
    "Podaj datę w formacie liczbowym: D:DD:MM:RRRR (gdzie: D - dzień tygodnia, DD - dzień miesiąca, MM - miesiąc, RRRR - rok)"];

pub const OUTPUT_FORMAT_SELECTION_PROMPT: [&str; 2] = [
    "Choose output format: US or EU",
    "Wybiesz format daty: US (amerykański) or EU (europejski)",
];

pub const WEEKDAYS: [[&str;7]; 2] = [
    [
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
        "Sunday",
    ],
    [
        "Poniedziałek",
        "Wtorek",
        "Środa",
        "Czwartek",
        "Piątek",
        "Sobota",
        "Niedziela",
    ],
];

pub const MONTHS: [[&str; 12]; 2] = [
    [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ],
    [
        "Styczeń",
        "Luty",
        "Marzec",
        "Kwiecień",
        "Maj",
        "Czerwiec",
        "Lipiec",
        "Sierpień",
        "Wrzesień",
        "Październik",
        "Listopad",
        "Grudzień",
    ],
];