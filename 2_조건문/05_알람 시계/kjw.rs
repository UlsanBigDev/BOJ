use std::io;

fn main() {
    let mut a = String::new();

    io::stdin().read_line(&mut a).unwrap();

    let anum: Vec<i32> = a
        .as_mut_str()
        .split_whitespace()
        .map(|a| a.parse().unwrap())
        .collect();

    let h: i32 = anum[0];
    let m: i32 = anum[1];

    let (h, m) = match (h, m) {
        (0..=23, 45..=59) => (h, m - 45),
        (0, 0..=44) => (23, 60 - (45 - m)),
        (1..=23, 0..=44) => (h - 1, 60 - (45 - m)),
        _ => unreachable!(), //컴파일 에러 시
    };
    print!("{} {}", h, m);
}
