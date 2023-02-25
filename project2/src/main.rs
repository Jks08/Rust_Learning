// Main function is always the first function to be executed
// It is the entry point of the program
// Rust uses snake case for naming functions and variables
// Fucntion parameters needs to be declared with their type
// In function, if return is not typed and semi-colon is not used, the function 
// will return ipmlicitly throw the last expression
// println!() is a macro, not a function. It outputs number after putting {}

// Some Rules:
// 1. Each value in Rust is owned by a variable
// 2. When an owner goes out of scope, the value will be deallocated
// 3. There can only be one owner at a time - Eliminates double free errors

use std::io;

fn main() {
    println!("Enter youe Earth weight in kg: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let weight: f64 = input.trim().parse().unwrap();
    let mars_weight = calc_weight_on_mars(weight);
    println!("Mars Weight: {} kg", mars_weight);
}

fn calc_weight_on_mars(weight:f64) -> f64 {
    (weight / 9.81) * 3.711
}