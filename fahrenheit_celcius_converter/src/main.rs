//Fahrenheit and celcius converter

use std::io;

fn main() {

    let mut value :f64 = 0.0;
    println!("Fahrenheit / Celcius Converter");

    'value_loop: loop {

        println!("Enter your value: ");

        let mut input = String::new();

        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read user input");

        value = match input.trim().parse() {
            Ok(float) => float,
            Err(_) => {
                println!("Invalid input. Expected float");
                continue 'value_loop;
            }
        };
        break;
    }
        
    let celcius = ftoc(value);
    let fahrenheit = ctof(value);
    println!("Converted to Celcius: {celcius}");
    println!("Converted to Fahrenheit: {fahrenheit}");
    
}


fn ftoc (fahrenheit :f64) -> f64 {
    (fahrenheit - 32.0) / 1.8
}

fn ctof (celcius :f64) -> f64 {
    (celcius * 1.8) + 32.0
}

