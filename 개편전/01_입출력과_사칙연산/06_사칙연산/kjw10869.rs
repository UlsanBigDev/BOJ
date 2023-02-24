use std::io;

fn main() {
    let mut input_number = String::new();
    
    io::stdin().read_line(&mut input_number).expect("fuck error");
    
    let numbers:Vec<&str> = input_number.split_whitespace().collect();
    
    let a = numbers[0].parse::<i32>().unwrap();
    
    let b = numbers[1].parse::<i32>().unwrap();
    
    println!("{}", a + b);
    println!("{}", a - b);
    println!("{}", a * b);
    println!("{}", a / b);
    println!("{}", a % b);
}
