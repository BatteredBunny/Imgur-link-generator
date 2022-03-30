use std::io::{stdout, Write};
use std::sync::atomic::{AtomicU16, Ordering};
use std::sync::{Arc, mpsc};
use std::thread;

use clap::Parser;
use curl::easy::Easy;

use crate::spinner::Spinner;
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
    Valid(String)
}

fn main() {
    let args = Args::parse();
    let mut spinner: Spinner = Spinner::new();

    let (sender, receiver) = mpsc::channel::<StatusReport>();

    let mut attempt: u32 = 0;
    let found = Arc::new(AtomicU16::new(0));

    let mut threads:Vec<thread::JoinHandle<()>> = Vec::new();

    for _ in 0..args.threads {
        let found_amount = Arc::clone(&found);
        let status_sender = sender.clone();

        threads.push(thread::spawn(move || {
                let mut curl = Easy::new();
                let mut generator = CodeGenerator::new();

                loop {
                    if found_amount.load(Ordering::Relaxed) >= args.amount {
                        break
                    }

                    let url: String = generator.generate(args.length.into());

                    curl.url(&url).unwrap();
                    if curl.perform().is_err() {
                        continue
                    }

                    let response_code: u32 = curl.response_code().unwrap();
                    match response_code {
                        429 => panic!("You seemed to have been blocked!"),
                        302 => status_sender.send(Invalid(url)).unwrap(), // Invalid image
                        200 => status_sender.send(Valid(url)).unwrap(), // Valid image
                        _ => {}
                    };
                }
            }));
    }

    // Code receiver
    loop {
        if let Ok(report) = receiver.recv() {
            match report {
                Valid(url) => {
                    println!("\r{}                                   ", url);
                    stdout().flush().unwrap();

                    found.fetch_add(1, Ordering::Relaxed);
                    attempt = 0;
                },
                Invalid(url) => {
                    if args.tries {
                        print!("\r{} Attempt {} ({})", spinner.next(), &attempt, &url);
                    } else {
                        print!("\r{} {}", spinner.next(), &PROGRESS_TEXT);
                    }

                    stdout().flush().unwrap();
                    attempt += 1;
                }
            }
        }

        if found.load(Ordering::SeqCst) >= args.amount {
            break
        }
    }

    for thread in threads {
        let _ = thread.join();
    }
}
