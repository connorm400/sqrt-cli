/// figures out the square root of a number to a certain number of digits (accuracy.)
/// 
/// # panics
/// this function will panic if the accuracy is less than 0.
pub fn sqrt_aprox(n: f64, accuracy: u32) -> f64 {
    assert!(accuracy > 0);
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
/// Works with any type that implements Ordering and subtraction (as long as the answer is of the same type)
//I really only made this to clean up code
pub fn difference<T>(a: T, b: T) -> T 
where T: PartialOrd + Sub<Output = T> {
    if a > b {
        a-b
    } else {
        b-a
    }
}