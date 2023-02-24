fn main() {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    
    stdin.read_line(&mut buf).unwrap();
    let buf = buf.trim().split_whitespace().map(|v| v.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let (cost1, cost2, price) = (buf[0], buf[1], buf[2]);

    if price - cost2 <= 0{
        print!("-1");
    }else{
        print!("{}", cost1 / (price - cost2) as i32 + 1);
    }
}
