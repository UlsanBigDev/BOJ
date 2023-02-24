use std::io;
fn main() {
    let mut a = String::new();

    io::stdin().read_line(&mut a).expect("error");

    let num:i32 = a.trim().parse().expect("numer");

    for i in 1..=9{
        println!("{} * {} = {}", num, i, num * i);
    }
}
