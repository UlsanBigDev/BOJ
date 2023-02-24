fn main(){
    let mut buf = String::new();
    let stdin = std::io::stdin();

    stdin.read_line(&mut buf).unwrap();
    let mut n = buf.trim().parse::<i32>().unwrap();
    
    let mut i = 2;
    while n >= i{
        if n % i == 0{
            n /= i;
            println!("{i}");
        }else{
            i += 1;
        }
    }
}
