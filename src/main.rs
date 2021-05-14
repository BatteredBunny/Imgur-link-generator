use rand::{thread_rng, Rng};
use std::io::{stdout, Write};
use clap::{Arg, App};
use curl::easy::Easy;

use std::thread;
use std::sync::mpsc;

fn generate_code(code_length: i32, extension: &str) -> String {
    let mut rng = thread_rng();

    const SITE_NAME: &str = "https://i.imgur.com/";
    const CHAR_POOL: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

    let code: String = (0..code_length)
    .map(|_| {
        let thing = rng.gen_range(0..CHAR_POOL.len());
        CHAR_POOL[thing] as char
    }).collect();
    let full_code = format!("{}{}{}", SITE_NAME, code, extension);
    return full_code;
}

fn progress_spinnerino(index: usize, animation: &str, secondary_text: &str) {
    let animation_length = animation.chars().count();
    let animation_cycle  = animation.chars().nth(index % animation_length).unwrap();

    print!("\r{} {}", animation_cycle, secondary_text);
    stdout().flush().unwrap();
}

fn main() {
    let matches = App::new("imgur link generator")
    .version("0.1.1")
    .arg(Arg::with_name("length")
        .short("l")
        .long("length")
        .value_name("Int")
        .default_value("5")
        .help("Changes generated imgur code lenght")
        .takes_value(true)
    ).arg(Arg::with_name("amount")
        .short("a")
        .long("amount")
        .value_name("Int")
        .default_value("1")
        .help("How many codes to generate")
    ).arg(Arg::with_name("tries")
        .short("t")
        .long("tries")
        .value_name("Bool")
        .default_value("false")
        .help("Shows failed tries")
    ).arg(Arg::with_name("raw")
        .short("r")
        .long("raw")
        .value_name("Bool")
        .default_value("false")
        .help("Sends raw results")
)   .get_matches();

    let code_length: i32 = matches.value_of("length").unwrap()
        .parse::<i32>()
        .unwrap();

    let amount: i32 = matches.value_of("amount").unwrap()
        .parse::<i32>()
        .unwrap();

    let show_tries: bool = matches.value_of("tries").unwrap()
        .parse::<bool>()
        .unwrap();

    let raw: bool = matches.value_of("raw").unwrap()
        .parse::<bool>()
        .unwrap();

    let extension: &str = ".jpg";
    let spinner = "⣾⣽⣻⢿⡿⣟⣯⣷";
    let progress_text = "Generating code";

    for _ in 1..=amount {

        let (tx1, rx) = mpsc::channel();
        let tx2 = tx1.clone();
        let tx3 = tx1.clone();
        let tx4 = tx1.clone();

        let mut response_code = 302;

        let mut index: i32 = 0;

        let thread1 = thread::spawn(move || {
            let mut easy = Easy::new();
            let mut code = String::new();

            while response_code == 302 && response_code != 429 {
                if !raw {
                    if show_tries {
                        let failed_text = format!("Attempt {} ({})", index, &code);
                        progress_spinnerino(index as usize, spinner, &failed_text);
                    } else {
                        progress_spinnerino(index as usize, spinner, &progress_text);
                    }
                        index += 1;
                }
    
                code = generate_code(code_length, extension);
    
                easy.url(&code).unwrap();
                easy.perform().unwrap();
    
                response_code = easy.response_code().unwrap();
            }
            if response_code == 429 {
                panic!("You seemed to have been blocked!");
            }

            tx1.send(code).unwrap();
        });
        let thread2 = thread::spawn(move || {
            let mut easy = Easy::new();
            let mut code = String::new();

            while response_code == 302 && response_code != 429 {
                if !raw {
                    if show_tries {
                        let failed_text = format!("Attempt {} ({})", index, &code);
                        progress_spinnerino(index as usize, spinner, &failed_text);
                    } else {
                        progress_spinnerino(index as usize, spinner, &progress_text);
                    }
                        index += 1;
                }
    
                code = generate_code(code_length, extension);
    
                easy.url(&code).unwrap();
                easy.perform().unwrap();
    
                response_code = easy.response_code().unwrap();
            }
            if response_code == 429 {
                panic!("You seemed to have been blocked!");
            }

            tx2.send(code).unwrap();
        });

        let thread3 = thread::spawn(move || {
            let mut easy = Easy::new();
            let mut code = String::new();

            while response_code == 302 && response_code != 429 {
                if !raw {
                    if show_tries {
                        let failed_text = format!("Attempt {} ({})", index, &code);
                        progress_spinnerino(index as usize, spinner, &failed_text);
                    } else {
                        progress_spinnerino(index as usize, spinner, &progress_text);
                    }
                        index += 1;
                }
    
                code = generate_code(code_length, extension);
    
                easy.url(&code).unwrap();
                easy.perform().unwrap();
    
                response_code = easy.response_code().unwrap();
            }
            if response_code == 429 {
                panic!("You seemed to have been blocked!");
            }

            tx3.send(code).unwrap();
        });

        let thread4 = thread::spawn(move || {
            let mut easy = Easy::new();
            let mut code = String::new();

            while response_code == 302 && response_code != 429 {
                if !raw {
                    if show_tries {
                        let failed_text = format!("Attempt {} ({})", index, &code);
                        progress_spinnerino(index as usize, spinner, &failed_text);
                    } else {
                        progress_spinnerino(index as usize, spinner, &progress_text);
                    }
                        index += 1;
                }
    
                code = generate_code(code_length, extension);
    
                easy.url(&code).unwrap();
                easy.perform().unwrap();
    
                response_code = easy.response_code().unwrap();
            }
            if response_code == 429 {
                panic!("You seemed to have been blocked!");
            }
            
            tx4.send(code).unwrap();
        });
        
        let final_code = rx.recv().unwrap();

        thread1.join().unwrap();
        thread2.join().unwrap();
        thread3.join().unwrap();
        thread4.join().unwrap();

        if !raw {
            stdout().flush().unwrap();
            print!("\r{}                                              \n", final_code);
        } else {
            println!("{}", final_code);
        }

    }
}
