use std;

fn main(){
    let mut text = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut text).unwrap();
    
    let n = text.trim().parse::<i32>().unwrap();
    for i in 1..10{
        println!("{} * {} = {}", n, i, n * i);
    }
}
