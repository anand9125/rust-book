use std::io;  //Imports the io module from Rust's standard library, which is used for input/output operations.

fn main() {  //which is the entry point of many programs
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();//it will store user input
     println!("index {}", index);
    io::stdin()     //This accesses the standard input stream (keyboard input).
        .read_line(&mut index)  //This method reads a full line from standard input and stores it in the variable index.
        .expect("Failed to read line");  //This handles the Result returned by read_line.If reading the line succeeds, the program continues. otherwise crashes

    let index: usize = index  //which is an unsigned integer type.
        .trim() // removes whitespace and newline characters.
        .parse()  // tries to convert the string into a usize.
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}

//Question: Why store input in a String if we expect a number? Why not read a number directly?
//Ans :Rust's stdin().read_line() only reads input as a String.There is no direct way in standard Rust to read a number (like usize) from input without first storing it as a string, and then parsing it.
//Rust’s standard input system is line-based, not token-based
//so when you do 
//io::stdin().read_line(&mut index)
// You're reading a line of text — always. Even if the user types 3, it's stored as "3\n".

// To use it as a number, you must:

// Trim it → remove \n and spaces.

// Parse it → convert to number type like usize.