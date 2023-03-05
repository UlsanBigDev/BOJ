fn main(){
    let mut buf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buf).unwrap();

    let n = buf.trim();

    print!("{}", n.len());
}
