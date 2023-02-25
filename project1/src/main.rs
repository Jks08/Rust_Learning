// fn main() {
//     println!("Hello, world!");
//     // Github copilot for Rust. 
// }

// Program to input 2 numbers and print their sum

// use std::io;

// fn main() {
//     let mut num1 = String::new();
//     let mut num2 = String::new(); 
//     println!("Enter first number");
//     io::stdin().read_line(&mut num1).expect("Failed to read line");
//     println!("Enter second number");
//     io::stdin().read_line(&mut num2).expect("Failed to read line");
//     let num1: i32 = num1.trim().parse().expect("Please type a number!");
//     let num2: i32 = num2.trim().parse().expect("Please type a number!");
//     println!("Sum of {} and {} is {}", num1, num2, num1 + num2);
// }

fn main(){
    let a =2;
    let result = stack_only(a);
    dbg!(result);
}

fn stack_only(b: i32) -> i32{
    let c = 3;
    return b+c + stack_and_heap();
}

fn stack_and_heap() -> i32{
    let d =5;
    let e = Box::new(10);
    return d + *e;
}
