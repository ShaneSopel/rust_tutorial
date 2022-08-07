// Rust Tutorial #5 - Console Input
use std::io;

fn main() {
    
    // How to take in an input..
    // type must be a string when we are tring to read in a line. 
    let mut input = String::new();
    // Expect helps us if we are trying to handle an error. 
    io::stdin().read_line(&mut input).expect("failed to read line");
    println!("{}", input);
}
