use rand::{thread_rng, Rng};
use std::io::{stdout, Write};
use clap::{Arg, App};
use curl::easy::Easy;

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
    .version("0.1.0")
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
    ).get_matches();

    let code_length: i32 = matches.value_of("length").unwrap()
        .parse::<i32>()
        .unwrap();

    let amount: i32 = matches.value_of("amount").unwrap()
        .parse::<i32>()
        .unwrap();

    let show_tries: bool = matches.value_of("tries").unwrap()
        .parse::<bool>()
        .unwrap();

    let extension: &str = ".jpg";
    let spinner = "⣾⣽⣻⢿⡿⣟⣯⣷";
    let progress_text = "Generating code";

    let mut easy = Easy::new();

    for mut num in 1..=amount {
        progress_spinnerino(num as usize, spinner, &progress_text);

        let mut code = generate_code(code_length, extension);

        easy.url(&code).unwrap();
        easy.perform().unwrap();

        let mut failed_amount: i32 = 0;

        while easy.response_code().unwrap() == 302 {
            if show_tries {
                failed_amount += 1;
                let failed_text = format!("Failed {} times. ({})", failed_amount, code);
                progress_spinnerino(num as usize, spinner, &failed_text);
            } else {
                progress_spinnerino(num as usize, spinner, &progress_text);
            }

            code = generate_code(code_length, extension);

            easy.url(&code).unwrap();
            easy.perform().unwrap();

            num += 1;
        }

        stdout().flush().unwrap();
    
        print!("\r{}                                              \n", code);
    }
}
