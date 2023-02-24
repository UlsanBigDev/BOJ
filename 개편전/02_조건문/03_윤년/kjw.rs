use std::io;

fn main() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();

    let result: i32 = a.trim().parse().unwrap();

    let s: bool = match result {
        result => result % 400 == 0 || (result % 4 == 0 && result % 100 != 0),
        _ => false,
    };

    match s {
        true => print!("1"),
        false => print!("0"),
    }
