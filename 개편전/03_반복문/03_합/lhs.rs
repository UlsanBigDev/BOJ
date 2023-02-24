use std;

fn main(){
    let mut n = String::new();

    let stdin = std::io::stdin(); //입력 받기
    stdin.read_line(&mut n).unwrap();
    
    let n = n.trim().parse::<i32>().unwrap();
    print!("{}", n * (n + 1) / 2);
}
