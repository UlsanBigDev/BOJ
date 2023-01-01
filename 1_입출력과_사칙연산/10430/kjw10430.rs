use std::io;

fn main() {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let a: Vec<i32> = s
        .as_mut_str()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    println!(
        "{}\n{}\n{}\n{}",
        (a[0] + a[1]) % a[2],
        ((a[0] % a[2]) + (a[1] % a[2])) % a[2],
        (a[0] * a[1]) % a[2],
        ((a[0] % a[2]) * (a[1] % a[2])) % a[2]
    );
}
