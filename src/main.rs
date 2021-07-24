use rand::{thread_rng, Rng};
use std::io::{stdout, Write};
use curl::easy::Easy;
use std::{thread, env};
use std::sync::mpsc;
use std::process::exit;

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
    let args: Vec<String> = env::args().collect();

    let mut code_length: i32 = 5;
    let mut amount: i32 = 1;
    let mut show_tries: bool = false;
    let mut raw: bool = false;

    let mut found_flag: bool = false;
    for i in 1..args.len() {
        if args[i] == "-h" || args[i] == "--help" {
            println!("imgur link generator 0.1.2");

            println!("USAGE:");
            println!("imgur_link_generator [OPTIONS]");

            println!("FLAGS:");
            println!("  -h, --help       Prints help information");
            println!("  -V, --version    Prints version information");

            println!("OPTIONS:");
            println!("  -a, --amount <Int>    How many codes to generate [default: 1]");
            println!("  -l, --length <Int>    Changes generated imgur code lenght [default: 5]");
            println!("  -r, --raw <Bool>      Sends raw results [default: false]");
            println!("  -t, --tries <Bool>    Shows failed tries [default: false]");

            exit(1)
        } else if args[i] == "-V" || args[i] == "--version" {
            println!("0.1.1");
            exit(1)
        } else if args[i] == "-l" || args[i] == "--lenght" {
            found_flag = true;
        } else if args[i] == "-a" || args[i] == "--amount" {
            found_flag = true;
        } else if args[i] == "-t" || args[i] == "--tries" {
            found_flag = true;
        } else if args[i] == "-r" || args[i] == "--raw" {
            found_flag = true;
        } else if found_flag {
            if args[i-1] == "-l" || args[i-1] == "--lenght" {
                code_length = args[i].parse::<i32>().unwrap();
            } else if args[i-1] == "-a" || args[i-1] == "--amount" {
                amount = args[i].parse::<i32>().unwrap();
            } else if args[i-1] == "-t" || args[i-1] == "--tries" {
                show_tries = args[i].parse::<bool>().unwrap();
            } else if args[i-1] == "-r" || args[i-1] == "--raw" {
                raw = args[i].parse::<bool>().unwrap();
            }

            found_flag = false;
        }
    }

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
