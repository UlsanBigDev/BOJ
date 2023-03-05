fn main(){
    let mut buf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buf).unwrap();

    let n = buf.trim().parse::<i32>().unwrap();
    buf.clear();

    for _ in 0..n{
        stdin.read_line(&mut buf).unwrap();
        let text = buf.trim().as_bytes();
        println!("{}{}", text[0 as usize] as char,text[(text.len()-1) as usize] as char);
        buf.clear();
    }
}
