use std::io::{stdout, Write};
use std::ptr::eq;
use std::sync::mpsc;
use std::thread;

use clap::Parser;
use curl::easy::Easy;

use crate::spinner::{Spinner, SpinnerChar, SpinnerKind};
use crate::StatusReport::{Invalid, Valid};

use code_generation::CodeGenerator;

mod spinner;
mod code_generation;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Shows failed tries [default: false]
    #[clap(long)]
    tries: bool,

    /// Generated imgur code length
    #[clap(short, long, default_value_t = 5)]
    length: u8,

    /// How many codes to generate
    #[clap(short, long, default_value_t = 1)]
    amount: u16,

    /// how many threads to run on
    #[clap(short, long, default_value_t = thread::available_parallelism().unwrap().get())]
    threads: usize,

}

const PROGRESS_TEXT: &str = "Generating code";

enum StatusReport {
    Invalid(String),
    Valid(String),
    Ping
}

fn main() {
    let args = Args::parse();
    let mut spinner: Spinner = Spinner::new();

    let (sender, receiver) = mpsc::channel::<StatusReport>();

    let mut attempt: u32 = 0;
    let mut found: u16 = 0;
    let mut threads:Vec<thread::JoinHandle<()>> = Vec::new();

    for _ in 0..args.threads {
        let status_sender = sender.clone();

        threads.push(thread::spawn(move || {
                let mut curl = Easy::new();
                let mut generator = CodeGenerator::new();

                while status_sender.send(StatusReport::Ping).is_ok() {
                    let url: String = generator.generate(args.length.into());

                    curl.url(&url).unwrap();
                    if curl.perform().is_err() {
                        continue
                    }

                    let response_code: u32 = curl.response_code().unwrap();
                    match response_code {
                        429 => panic!("You seemed to have been blocked!"),
                        302 => {
                            if status_sender.send(Invalid(url)).is_err() {
                                break
                            }
                        }, // Invalid image
                        200 => {
                            if status_sender.send(Valid(url)).is_err() {
                                break
                            }
                        }, // Valid image
                        _ => {}
                    };
                }
            }));
    }

    // Code receiver
    while found < args.amount {
        if let Ok(report) = receiver.recv() {
            match report {
                Valid(url) => {
                    println!("\r{}                                   ", url);
                    stdout().flush().unwrap();

                    found += 1;
                    attempt = 0;
                },
                Invalid(url) => {
                    let animation_frame = spinner.next();

                    if args.tries {
                        print!("\r{} Attempt {} ({})", animation_frame.ch, &attempt, &url);
                        stdout().flush().unwrap();
                    } else if let SpinnerKind::Ok = animation_frame.kind {
                        print!("\r{} {}", animation_frame.ch, &PROGRESS_TEXT);
                        stdout().flush().unwrap();
                    }

                    attempt += 1;
                }
                _ => ()
            }
        }
    }

    drop(sender);
    drop(receiver);

    for thread in threads {
        let _ = thread.join();
    }
}
