use std::iter::Cycle;
use std::str::Chars;
use std::time::{Duration, Instant};

pub struct Spinner<'a> {
    current: char,
    spinner: Cycle<Chars<'a>>,
    last_call: Instant
}

pub enum SpinnerResult {
    Ok(char),
    TooSoon(char)
}

impl<'a> Spinner<'a> {
    pub fn new() -> Self {
        Spinner {
            current: '⣾',
            spinner: "⣾⣽⣻⢿⡿⣟⣯⣷".chars().cycle(),
            last_call: Instant::now()
        }
    }

    pub fn next(&mut self) -> SpinnerResult {
        if self.last_call.elapsed() > Duration::from_millis(100) {
            self.last_call = Instant::now();
            self.current = self.spinner.next().unwrap();
            
            SpinnerResult::Ok(self.current)
        } else { 
            SpinnerResult::TooSoon(self.current)
        }
    }
}