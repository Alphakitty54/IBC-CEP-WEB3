use std::io;

fn main() {
    println!("Simple Calculator");
    println!("1. Addition");
    println!("2. Subtraction");
    println!("3. Multiplication");
    println!("4. Division");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    let choice: u32 = choice.trim().parse().expect("Please enter a valid number");

    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).expect("Failed to read line");
    let num1: f64 = num1.trim().parse().expect("Please enter a valid number");

    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).expect("Failed to read line");
    let num2: f64 = num2.trim().parse().expect("Please enter a valid number");

    match choice {
        1 => println!("Result: {}", add(&num1, &num2)),
        2 => println!("Result: {}", subtract(&num1, &num2)),
        3 => println!("Result: {}", multiply(&num1, &num2)),
        4 => println!("Result: {}", divide(&num1, &num2)),
        _ => println!("Invalid choice"),
    }
}

fn add(a: &f64, b: &f64) -> f64 {
    a + b
}

fn subtract(a: &f64, b: &f64) -> f64 {
    a - b
}

fn multiply(a: &f64, b: &f64) -> f64 {
    a * b
}

fn divide(a: &f64, b: &f64) -> f64 {
    a / b
}