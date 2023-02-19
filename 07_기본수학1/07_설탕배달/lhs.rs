fn main() {
    let mut buf = String::new();
    let stdin = std::io::stdin();

    stdin.read_line(&mut buf).unwrap();
    let n = buf.trim().parse::<i32>().unwrap();
    let mut k = (n / 5) as i32;

    loop {
        if (n - 5 * k) % 3 == 0{
            break;
        }
        k -= 1;
    }
    if k == -1{
        print!("-1");
    }else{
        println!("{}", k + ((n - 5 * k) /3) as i32);
    }
}   
