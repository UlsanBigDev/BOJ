fn main(){
    let mut buf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buf).unwrap();

    let n = buf.clone().trim().parse::<i32>().unwrap();
    buf.clear();
    stdin.read_line(&mut buf).unwrap();

    let n = buf.trim().chars().map(|v| v.to_digit(10).unwrap() as i32).collect::<Vec<i32>>().iter().sum::<i32>();
    println!("{:?}", n);
}
