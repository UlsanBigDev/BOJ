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
    
    stdin.read_line(&mut buf).unwrap();
    let numbers = buf.trim().split_whitespace().map(|v| v.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let mut result = 0;
    for (_, v) in numbers.iter().enumerate(){
        result += isPrime(v) as i32;
    }

    print!("{result}");
}
