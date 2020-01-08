// fibonacci!
// sum of the two preceeding numbers
// https://en.wikipedia.org/wiki/Fibonacci_number
// F0 = 0 , F1 = 1
// Fn = Fn-1 + Fn-2
//
// (0, 1, 1, 2, 3, 5, 8, 13, 21, etc
use std::io;

fn main() {

    // get desired count, cast to int
    let count = loop {
        println!("enter a number");

        let mut count = String::new();
        io::stdin().read_line(&mut count)
            .expect("failed to read line");

        let count: u32 = match count.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        break count;
    };

    // generate the sequence, and return the number, I guess?
    println!("fib number {} is {}", count, fibonacci(count));
}

// recursive function!
// https://benjaminbrandt.com/fibonacci-in-rust/

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n-1) + fibonacci(n-2),
    }
}
