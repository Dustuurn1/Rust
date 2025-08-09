//Prints the nth value of the fibonacci sequence

use std::io;

fn main() {
    println!("Enter the nth term of the fibonacci sequence: ");

    let mut value: u32 = 0;

    loop {
        let mut input = String::new();

        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read user input"); 

        value = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Expected integer");
                continue;
            }
        };
        break;
    }

    let suffix_mod = value % 10;
    let suffix = 
        if suffix_mod == 1 {"st"}
        else if suffix_mod == 2 {"nd"}
        else if suffix_mod == 3 {"rd"}
        else {"th"};

    let fib_val = fib(value);
    
    println!("The {value}{suffix} term of the fibonacci sequence is : {fib_val}");
    

}

fn fib (n: u32) -> u32 {
    if n == 0 || n == 1 {return n;}
    else {
        return fib(n-2) + fib(n-1);
    };
}