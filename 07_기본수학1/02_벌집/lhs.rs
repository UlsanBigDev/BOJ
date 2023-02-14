fn main() {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    
    stdin.read_line(&mut buf).unwrap();
    let n = buf.trim().parse::<i32>().unwrap();

    let mut i = 2 ;

    if n == 1{
        print!("1");
    }else{
        while true{
            if n <= 1 + i * (i-1) * 3{
                print!("{}", i);
                break;
            }
            i+= 1;
        }
    }
}
