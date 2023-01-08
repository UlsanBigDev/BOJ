use std::io;

fn main() {
    let mut input_number = String::new();

    io::stdin().read_line(&mut input_number)
        .expect("Falied");

    let numbers: Vec<&str> = input_number.split_whitespace().collect();

    let number_a = numbers[0].parse::<i32>().unwrap();
    
    let number_b = numbers[1].parse::<i32>().unwrap();
    
    println!("{}", number_a * number_b);

}
