use std::{io, usize};

fn main() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();

    let num: usize = a.trim().parse().unwrap(); //반복횟수

    for _ in 0..num {
        let mut b = String::new();
        io::stdin().read_line(&mut b).unwrap();
        let arr: Vec<&str> = b.trim().split_whitespace().collect(); //반복횟수와 문자열
        let cnt: usize = arr[0].parse().unwrap(); //횟수만 분리
        let ch: Vec<char> = arr[1].chars().collect(); // 문자열을 vec으로 만들어줌
        for c in ch.iter() {
            //문자열을 하나하나씩 c에 넣어주고 한번 반복할때마다 iter()로 위치를 옮겨줌
            for _ in 0..cnt {
                //반복횟수만큼 돌림
                print!("{}", c);
            }
        }
        print!("\n");
    }
}
