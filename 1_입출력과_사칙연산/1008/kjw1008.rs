use std::io;

fn main() {
    let mut input_number = String::new();
    
    io::stdin().read_line(&mut input_number).expect("Failed");
    
    let numbers: Vec<&str> = input_number.split_whitespace().collect();
    
    let number_a = numbers[0].parse::<f64>().unwrap();
    
    let number_b = numbers[1].parse::<f64>().unwrap();
    
    let result = number_a / number_b;
    
    println!("{}", result);
}
