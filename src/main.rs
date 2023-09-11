use sqrt_cli::{sqrt_aprox, parse_arguments};
use std::io::stdin;
use std::process;

fn main() {
    // error and exit if there isn't a -a flag or it has a bad number
    let accuracy = match parse_arguments::<u32>("-a") {
        Some(Ok(n)) => n,
        Some(Err(_e)) => {
            eprintln!("error parsing -a flag: {_e:?}");
            process::exit(1)
        }
        None => {
            eprintln!("didn't supply -a (accuracy) flag");
            process::exit(1)
        }
    };

    // read piped stdin
    let mut number = String::new();
    stdin().read_line(&mut number).expect("error reading stdin");
    number = number.trim().to_owned();

    // parse piped number to become a float
    let number = match number.parse::<f64>() {
        Ok(n) => n,
        Err(_e) => {
            eprintln!("piped input isn't a number");
            process::exit(1);
        }
    };

    if number < 0.0 {
        eprintln!("piped input must be more than 0");
        process::exit(1)
    }

    println!(
        "{}", sqrt_aprox(number, accuracy)
    );

    process::exit(0)
    
}