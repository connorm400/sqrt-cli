/// figures out the square root of a number to a certain number of digits (accuracy.)
/// 
/// Having a high accuracy (ex: 100) will make this function exponentially slower
/// 
/// # Examples
/// 
/// basic usage:
/// ```
/// use sqrt_cli::sqrt_aprox;
/// assert_eq!(2.0, sqrt_aprox(4.0, 1));
/// assert_eq!(1.4142156862745097 , sqrt_aprox(2.0, 5));
/// ```
pub fn sqrt_aprox(n: f64, accuracy: u32) -> f64 {
    let mut guess = n / 2.0;
    let accuracy = 10_f64.powi(0 - accuracy as i32);

    loop {
        guess = ((n / guess) + guess) / 2.0;

        if  (guess.powf(2.0) - n).abs() < accuracy {
            break;
        }
    }

    guess
}

use std::env::args;
use std::str::FromStr;
//use std::num::ParseIntError;
/// Will look through the program's arguments, looking for a specific 
/// flag (ex: `-a`). Then it will see if there is a proceeding number to parse.
/// 
/// Usefull for making handling command line arguments that need an integer value afterwards
/// 
/// ## Examples
/// 
/// basic usage:
/// ```
/// use sqrt_cli::parse_arguments;
/// // will return None, or Some(Ok(value)) or Some(Err(ParseIntError))
/// //depending on the arguments given when running the program
/// println!("{:?}", parse_arguments::<i32>("-c"))
/// ```
pub fn parse_arguments<T>(flag: &str) -> Option<Result<T, <T as FromStr>::Err>> 
where T: FromStr {
    args().enumerate()
        .find(|(_, x)| *x == flag.to_owned())
        .and_then(|(i, _)| {
            args().nth(i + 1).and_then(|n| {
                Some(n.parse::<T>())
            })
        })
}
