use std::iter::Cycle;
use std::str::Chars;

pub struct Spinner<'a> {
    spinner: Cycle<Chars<'a>>,
}

impl<'a> Spinner<'a> {
    pub fn new() -> Self {
        Spinner {
            spinner: "⣾⣽⣻⢿⡿⣟⣯⣷".chars().cycle(),
        }
    }

    pub fn next(&mut self) -> char {
        self.spinner.next().unwrap()
    }
}