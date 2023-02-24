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

    loop {
        stdin.read_line(&mut buf).unwrap();
        let n = buf.clone().trim().parse::<i32>().unwrap();
        
        if n == 0{
            break;
        }

        let mut result = 0;
        for i in n+1..=(2*n){
            result += isPrime(&i) as i32;
        }
        println!("{result}");
        buf.clear();
    }
}
