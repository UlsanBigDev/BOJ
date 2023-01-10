use std::io;
fn main() {
    let mut a = String::new();

    io::stdin().read_line(&mut a).expect("error");

    let num:i32 = a.trim().parse().expect("numer");

    print!("{}", num * (num + 1) / 2);
}
