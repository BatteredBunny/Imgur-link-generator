#![feature(test)]

use std::io::{stdout, Write};
use std::thread::available_parallelism;

use clap::Parser;
use reqwest::redirect::Policy;
use tokio::sync::mpsc::unbounded_channel;
use tokio::task::JoinHandle;

use crate::code_generation::CodeGenerator;
use crate::spinner::{Spinner, SpinnerKind};
use crate::StatusReport::{Invalid, Valid};

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
    #[clap(short, long, default_value_t = available_parallelism().unwrap().get())]
    threads: usize,

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

            let _: JoinHandle<Result<(), reqwest::Error>> = tokio::spawn(async move {
                let mut generator = CodeGenerator::new(args.length as usize);
                let url = generator.generate();
                let resp = client.get(url).send().await?;

                match resp.status().as_u16() {
                    429 => panic!("You seemed to have been blocked!"),
                    302 => { // Invalid image
                        let _ = loop_sender.send(Invalid(url.to_string()));
                    },
                    200 => { // Valid image
                        let _ = loop_sender.send(Valid(url.to_string()));
                    },
                    _ => {}
                };

                Ok(())
            });
        }
    });

    // Code receiver
    while found < args.amount {
        if let Some(report) = receiver.recv().await {
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
            }
        }
    }

    drop(closer_sender);
    drop(receiver);
}
