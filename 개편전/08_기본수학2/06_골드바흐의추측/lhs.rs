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
    let n = buf.clone().trim().parse::<i32>().unwrap();
    buf.clear();

    for _ in 0..n{
        stdin.read_line(&mut buf).unwrap();
        let a = buf.clone().trim().parse::<i32>().unwrap();
        let mut list : [i32; 2] = [0, 0];

        for i in 0..=(a/2){
            if isPrime(&i) && isPrime(&(a - i)){
                list = [i, a-i];
            }
        }

        println!("{} {}",list[0], list[1]);
        buf.clear();
    }
}
