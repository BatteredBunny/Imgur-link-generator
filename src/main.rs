#![feature(test)]

use std::io::{stdout, Write};

use crate::{
    code_generation::CodeGenerator,
    spinner::{Spinner, SpinnerResult},
};
use clap::Parser;
use reqwest::redirect::Policy;
use tokio::sync::mpsc::unbounded_channel;

mod code_generation;
mod spinner;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Generated imgur code length [default: 5]
    #[clap(short, long, default_value_t = 5)]
    length: usize,

    /// How many codes to generate [default: 1]
    #[clap(short, long, default_value_t = 1)]
    amount: u16,

    /// Enables progress spinner [default: true]
    #[clap(short, long, default_value_t = true)]
    spinner: bool,

    /// Shows failed tries in spinner [default: false]
    #[clap(long)]
    tries: bool,
}
const PROGRESS_TEXT: &str = "Generating code";

enum StatusReport {
    Invalid(String),
    Valid(String),
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let mut spinner: Spinner = Spinner::new();

    let (sender, mut receiver) = unbounded_channel::<StatusReport>();

    let mut bad: u32 = 0;
    let mut attempt: u32 = 0;
    let mut found: u16 = 0;
    let closer_sender = sender.clone();

    tokio::spawn(async move {
        while !sender.is_closed() {
            let loop_sender = sender.clone();
            let client = reqwest::Client::builder()
                .redirect(Policy::none())
                .build()
                .unwrap();

            let _: Result<Result<(), reqwest::Error>, _> = tokio::spawn(async move {
                let mut generator = CodeGenerator::new(args.length);
                let url = generator.generate();
                let resp = client.head(url).send().await?;

                match resp.status().as_u16() {
                    429 => panic!("You seemed to have been blocked!"),
                    302 => {
                        // Invalid image
                        if loop_sender
                            .send(StatusReport::Invalid(url.to_string()))
                            .is_err()
                        {
                            return Ok(());
                        }
                    }
                    200 | 206 => {
                        // Valid image
                        if loop_sender
                            .send(StatusReport::Valid(url.to_string()))
                            .is_err()
                        {
                            return Ok(());
                        }
                    }
                    _ => {}
                };

                Ok(())
            })
            .await;
        }
    });

    // Code receiver
    while found < args.amount {
        if let Some(report) = receiver.recv().await {
            match report {
                StatusReport::Valid(url) => {
                    println!("\r{url}                                   ");
                    stdout().flush().unwrap();

                    found += 1;
                    attempt = 0;
                }
                StatusReport::Invalid(url) => {
                    if args.spinner {
                        match spinner.next() {
                            SpinnerResult::TooSoon(ch) => {
                                if args.tries {
                                    print!("\r{ch} Attempt {attempt} ({url})");
                                    stdout().flush().unwrap();
                                }
                            }
                            SpinnerResult::Ok(ch) => {
                                if args.tries {
                                    print!("\r{ch} Attempt {attempt} ({url})");
                                } else {
                                    print!("\r{ch} {PROGRESS_TEXT}");
                                }

                                stdout().flush().unwrap();
                            }
                        }
                    }

                    attempt += 1;
                    bad += 1;
                }
            }
        }
    }

    drop(closer_sender);
    drop(receiver);

    println!("OK: {found}, BAD: {bad}");
    std::process::exit(0)
}
