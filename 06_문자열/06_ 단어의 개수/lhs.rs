fn main() {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buf).unwrap();

    let n = buf.clone().trim().split_whitespace().count();

    print!("{}", n);
}
