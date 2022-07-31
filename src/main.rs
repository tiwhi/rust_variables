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
    let _spaces = String::new();
    println!("input as many spaces as you want to have counted.");
    // io::stdin()
    //     .read_line(&mut spaces)
    //     .expect("Error reading input");
    // let spaces = spaces.len();
    // println!("The number of spaces in the spaces variable is {spaces}");

    println!("Testing output of division:");
    let quotient = 30.0 / 10.5;
    let floored = 2 / 3;
    let remainder = 45 % 5;
    println!("Quotient: {quotient}, floored: {floored}, remainder: {remainder}");
    
    println!("Booleans =================================================================");
    let t = true;
    let f: bool = false;
    println!("t: {t}, f: {f}");

    println!("Characters =================================================================");
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("c: {c}, z: {z}, heart_eyed_cat: {heart_eyed_cat}");

    println!("Tuples =================================================================");
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tup: {:?}", tup);
    println!("Desctructing tuples:");
    let (x, y, z) = tup;
    println!("The value of x is: {x}, y is: {y}, z is: {z}");
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("The value of five_hundred is: {five_hundred}, six_point_four is: {six_point_four}, one is: {one}");

    println!("Arrays =================================================================");
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // delcare an array of 5 integers of type i32
    let mut b: [&str; 5] = ["a"; 5]; // delcare an array of 5 strings

    println!("Array a: {:?}", a);
    println!("Array b: {:?}", b);
    b[0] = "b";

    // Accessing Array Elements
    println!("Array a[0] should be 1 and is: {:?}", a[0]);
    println!("Array b[0] should be b and is: {:?}", b[0]);

    let guess_array = [1, 2, 3, 4, 5];

    println!("Please enter an array index");

    let mut index = String::new();

    io::stdin() // get the input from the user
        .read_line(&mut index) // push that input to the index variable
        .expect("Error reading input"); // panic on error
    
    let index: usize = index // converting the index into an int 
        .trim() // strip whitespaces
        .parse() // convert the index into an int 
        .expect("Index entered was not a number"); // panic on error 
    
    let element = guess_array[index]; // set the element variable to the value of array a at index in index variable

    println!("The value of the element at index {index} is {element}");
}
