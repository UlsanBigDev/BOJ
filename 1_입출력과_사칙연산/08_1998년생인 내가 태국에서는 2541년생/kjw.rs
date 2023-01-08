use std::io;

fn main() {
    let mut a = String::new();

    io::stdin().read_line(&mut a).unwrap();

    let result: i32 = a.trim().parse().unwrap();

    println!("{}", result - 543);
}
