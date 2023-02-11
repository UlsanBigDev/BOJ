fn main() {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buf).unwrap();

    let text = buf.trim().split_whitespace().collect::<Vec<&str>>();

    let n = text[0].bytes().rev().map(|v| v as char).collect::<String>();
    let e = text[1].bytes().rev().map(|v| v as char).collect::<String>();

    println!("{}", e.max(n));
}
