use std;

fn main(){

    let stdin = std::io::stdin();
    stdin.read_line(&mut buf).unwrap();

    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();

    let line = line.split_whitespace().map(|x| x.parse::<i32>().unwrap());
    let (max, min) = (line.clone().max().unwrap(), line.min().unwrap());
    
    print!("{} {}", min, max);
}
