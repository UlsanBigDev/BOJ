fn main(){
    let mut buf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buf).unwrap();

    let n = buf.trim().parse::<i32>().unwrap()/4;

    for _ in 0..n{
        print!("long ");
    }

    print!("int");
}
