fn main(){
    let mut buf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buf).unwrap();
    
    let list = buf.trim().split_whitespace().map(|v|v.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    print!("{}", list[0] + list[1] + list[2]);
}
