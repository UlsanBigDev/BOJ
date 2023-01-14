use std;

fn main(){

    let stdin = std::io::stdin();
    stdin.read_line(&mut n).unwrap();

    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    let line = line.split_whitespace().map(|x| x.parse::<i32>().unwrap());

    let mut num = String::new();
    stdin.read_line(&mut num).unwrap();
    let num = num.trim().parse::<i32>().unwrap();
    print!("{}", line.filter(|&y| y == num).count());
}
