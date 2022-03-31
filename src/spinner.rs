use std::iter::Cycle;
use std::str::Chars;
use std::time::{Duration, Instant};

pub struct Spinner<'a> {
    spinner: Cycle<Chars<'a>>,
    last_call: Instant
}

pub struct SpinnerChar {
    pub ch: char,
    pub kind: SpinnerKind
}

pub enum SpinnerKind {
    Ok,
    TooSoon
}

impl<'a> Spinner<'a> {
    pub fn new() -> Self {
        Spinner {
            spinner: "⣾⣽⣻⢿⡿⣟⣯⣷".chars().cycle(),
            last_call: Instant::now()
        }
    }

    pub fn next(&mut self) -> SpinnerChar {
        if self.last_call.elapsed() > Duration::from_millis(100) {
            self.last_call = Instant::now();
            SpinnerChar {
                ch: self.spinner.next().unwrap(),
                kind: SpinnerKind::Ok
            }
        } else {
            SpinnerChar {
                ch: self.spinner.clone().take(1).next().unwrap(),
                kind: SpinnerKind::TooSoon
            }
        }
    }
}