use sqrt_cli::sqrt_aprox;
use std::io::stdin;
use std::num::ParseIntError;
use std::env::args;
use std::process;

fn main() {
    let accuracy = match parse_arguments() {
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

    let mut number = String::new();
    stdin().read_line(&mut number).expect("error reading stdin");
    number = number.trim().to_owned();

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

// I don't know if this is the worst code I've ever written or the best
fn parse_arguments() -> Option<Result<u32, ParseIntError>> {
    args().enumerate()
        .find(|(_, x)| *x == "-a".to_owned())
        .and_then(|(i, _)| {
            args().nth(i + 1).and_then(|n| {
                Some(n.parse::<u32>().map_err(|e| return e))
            })
        })
}
