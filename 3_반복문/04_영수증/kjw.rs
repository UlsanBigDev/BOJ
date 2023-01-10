use std::io;
fn main() {
    let mut a = String::new();
    let mut b = String::new();
    io::stdin().read_line(&mut a).expect("error");
    io::stdin().read_line(&mut b).expect("error");

    let price: i32 = a.trim().parse().unwrap(); //총금액
    let num: i32 = b.trim().parse().unwrap(); //반복횟수
    let mut result: i32 = 0;
    for _ in 0..num {
        let mut don = String::new();
        io::stdin().read_line(&mut don).expect("error");

        let s: Vec<i32> = don
            .as_mut_str()
            .split_whitespace()
            .map(|don| don.parse().unwrap())
            .collect();

        result = result + s[0] * s[1];
    }

    let answer = match result == price {
        true => "Yes",
        false => "No",
    };

    print!("{}", answer);
}
