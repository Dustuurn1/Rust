//TODO
//create calculate function
//add cli params

use std::io;

fn main() {

    println!("Input your operation");
    let input = get_input();
    let mut inputs = input.split_whitespace();
    let mut numbers = Vec::new();
    let mut operations = Vec::new();

    while let Some(token) = inputs.next() {
        if let Ok(num) = token.trim().parse::<f32>() {
            numbers.push(num);
        } 
        else if matches!(token, "+" | "-" | "*" | "/") {
            operations.push(token);
        }
        else {
            continue;
        }
    }

    println!("Numbers: {}",
     numbers.iter()
     .map(|n| n.to_string())
     .collect::<Vec<_>>()
     .join(" "));

    println!("Operations: {}",
     operations.iter()
     .map(|n| n.to_string())
     .collect::<Vec<_>>()
     .join(" "));

    
}

fn get_input() -> String {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer) 
        .expect("Failed to get input");
    return buffer;
}

fn calculate(x : f32, y : f32, o : char) -> f32 {
    match o {
    '+' => x + y,
    '-' => x - y,
    '*' => x * y,
    //Division
    _ => 0.0}
}

