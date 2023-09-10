use sqrt_cli::sqrt_aprox;
use std::io::stdin;
use std::num::ParseIntError;
use std::env::args;
use std::process;

fn main() {
    // error and exit if there isn't a -a flag or it has a bad number
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

// I don't know if this is the worst code I've ever written or the best
// it'll look at all the arguments, find a -a flag, see if there is a
// number to parse and try to parse it. It'll return none if it can't find
// -a or the following number, and will return Some(Err(ParseIntError)) if the
// "number" wont parse to a unsigned int
fn parse_arguments() -> Option<Result<u32, ParseIntError>> {
    args().enumerate()
        .find(|(_, x)| *x == "-a".to_owned())
        .and_then(|(i, _)| {
            args().nth(i + 1).and_then(|n| {
                Some(n.parse::<u32>().map_err(|e| return e))
            })
        })
}
