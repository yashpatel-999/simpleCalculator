use std::io;

fn main() {
    let mut a = String::new();
    println!("Enter first Number:");
    io::stdin().read_line(&mut a).expect("Failed to read line");
    let a: i32 = a.trim().parse().expect("Please type a number!");
    
    let mut b = String::new();
    println!("Enter second Number:");
    io::stdin().read_line(&mut b).expect("Failed to read line");
    let b: i32 = b.trim().parse().expect("Please type a number!");
    
    let mut op = String::new();
    println!("Enter the operator (+, -, *, /):");
    io::stdin().read_line(&mut op).expect("Failed to read line");
    
    let op = op.trim();
    if op == "+" {
        println!("Result: {}", a + b);
    } else if op == "-" {
        println!("Result: {}", a - b);
    } else if op == "*" {
        println!("Result: {}", a * b);
    } else if op == "/" {
        if b != 0 {
            println!("Result: {}", a / b);
        } else {
            println!("Error: Division by zero is not allowed.");
        }
    } else {
        println!("Error: Invalid operator.");
    }
}