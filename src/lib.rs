/// figures out the square root of a number to a certain number of digits (accuracy.)
/// 
/// Having a high accuracy (ex: 100) will make this function exponentially slower
/// 
/// # Examples
/// 
/// basic usage:
/// ```
/// assert_eq!(2, sqrt_aprox(4, 1));
/// assert_eq!(1.4142156862745097 , sqrt_aprox(2.0, 5));
/// ```
pub fn sqrt_aprox(n: f64, accuracy: u32) -> f64 {
    let mut guess = n / 2.0;
    let accuracy = 10_f64.powi(0 - accuracy as i32);

    loop {
        guess = ((n / guess) + guess) / 2.0;

        if difference(guess.powf(2.0), n) < accuracy {
            break;
        }
    }

    guess
}

use std::ops::Sub;
/// C/C++'s fdim function. Will give the positive difference between two numbers. 
/// Works with any type that implements Ordering and subtraction (as long as the answer is of the same type).
/// Pretty self explanatory
//I really only made this to clean up code
pub fn difference<T>(a: T, b: T) -> T 
where T: PartialOrd + Sub<Output = T> {
    if a > b {
        a-b
    } else {
        b-a
    }
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
