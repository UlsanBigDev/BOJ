use std;

fn main(){
    let mut n = String::new();

    let stdin = std::io::stdin(); //입력 받기
    stdin.read_line(&mut n).unwrap();
    let n = n.trim().parse::<i32>().unwrap();

    for _ in 0..n {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();

        let test : i32= line.split_whitespace()
                            .map(|x| x.parse::<i32>().unwrap())
                            .sum();

        println!("{}", test);
    }
}
