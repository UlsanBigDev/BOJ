use std::{self, io::{Write, BufWriter}};

fn isPrime(n : &i32) -> bool{
    let i = (*n as f64).sqrt() as i32;

    let mut k = 2;

    if *n == 2{
        return true;
    }
    if *n == 1{
        return false;
    }

    while k <= i{
        if n % k == 0{
            return false;
        }
        k += 1;
    }
    return true;
}

fn main(){
    let mut buf = String::new();
    let stdin = std::io::stdin();

    stdin.read_line(&mut buf).unwrap();
    let t = buf.trim().split_whitespace().map(|v| v.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let (mut a, b) = (t[0], t[1]);

    let stdout = std::io::stdout();
    let mut writer = BufWriter::new(stdout.lock());
    
    while b >= a{
        if isPrime(&a) {writeln!(writer, "{a}").unwrap()} else {a+=1;continue}
        a+=1;
    }
}
