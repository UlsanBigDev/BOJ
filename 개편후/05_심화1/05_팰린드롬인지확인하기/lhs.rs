fn main(){
    let mut buf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buf).unwrap();

    let text = buf.trim().to_string();

    let rev_t = text.chars().rev().collect::<String>();

    print!("{}", if text == rev_t {1} else {0});
}
