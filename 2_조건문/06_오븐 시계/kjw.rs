use std::io;

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    io::stdin().read_line(&mut a).unwrap();
    io::stdin().read_line(&mut b).unwrap();
    let anum: Vec<i32> = a
        .as_mut_str()
        .split_whitespace()
        .map(|a| a.parse().unwrap())
        .collect();

    let bnum: i32 = b.trim().parse().unwrap(); //조리하는데 걸리는 시간

    let h: i32 = anum[0]; //시
    let m: i32 = anum[1]; //분

    let min: i32 = 60 * h + m; //시간을 분으로 전환(1시간은 60분)
    let cook: i32 = min + bnum; //조리한 뒤 시간을 분으로 환산

    let hour: i32 = (cook / 60) % 24; // 조리한 뒤 시간(분 이었던것을 시간으로)
    let minute: i32 = cook % 60; // 조리한 뒤 시간의 분

    print!("{} {}", hour, minute)
}
