use std;

fn main(){
    let mut buf = String::new();

    let stdin = std::io::stdin();
    stdin.read_line(&mut buf).unwrap();
    
    while !buf.is_empty(){
        println!("{}", buf.split_whitespace().map(|x| x.parse::<i32>().unwrap()).sum::<i32>());
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
    }
}
