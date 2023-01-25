use std::io;
fn main() {
    let mut a = String::new();

    io::stdin().read_line(&mut a).expect("error");

    let num:i32 = a.trim().parse().expect("numer");

    for _ in 0..num{
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("error");
        let snum:Vec<i32> = s.as_mut_str().split_whitespace().map(|s| s.parse().unwrap()).collect();

        println!("{}", snum[0] + snum[1]);
    }
}
