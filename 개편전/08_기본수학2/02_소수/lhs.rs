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
    let a = buf.clone().trim().parse::<i32>().unwrap();
    buf.clear();
    
    stdin.read_line(&mut buf).unwrap();
    let b = buf.trim().parse::<i32>().unwrap();

    let mut result = 0;

    let mut first = true;
    let mut min = 0;

    for t in a..=b{
        if first{
            if isPrime(&t){
                min = t;
                first = false;
            }
        }
        result += if isPrime(&t) {t} else {0};
        //if isPrime(&t) {println!("{t}")} else {continue}
    }
    
    if result == 0{
        print!("-1");
    }else{
        println!("{result}");
        print!("{min}");
    }
}
