use std::io;

fn main() {
    let x = 5;
    println!("The value of the x is {x}");
    let x = x + 1; 
    {
        let x = x * 2;
        println!("The value of the x in the inner scope is {x}");
    }
    println!("The value of the x in the outer scope is {x}");
    let mut spaces = String::new();
    println!("input as many spaces as you want to have counted.");
    io::stdin()
        .read_line(&mut spaces)
        .expect("Error reading input");
    println!("The number of spaces in the spaces variable is {spaces}");
}
