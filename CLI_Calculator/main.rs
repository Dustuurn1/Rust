use std::io;

fn main() {

    println!("Input your operation");
    let input = get_input();
    let mut inputs = input.split_whitespace();
    let mut numbers = Vec::new();

    while let Some(token) = inputs.next() {
        numbers.push(to_int(Some(token)));
    }

    println!("You entered: {}",
     numbers.iter()
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

fn to_int( s : Option<&str>) -> i32 {
    return s.unwrap().trim().parse().expect("Input not an integer");
}

