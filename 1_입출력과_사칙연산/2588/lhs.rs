fn main(){
    let mut a: String = String::new();
    let mut b: String = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut a);
    stdin.read_line(&mut b);

    let _a : i32 = a.trim().parse().unwrap();
    let _b : Vec<i32> = b.trim().chars().map(|x| x.to_digit(10).unwrap() as i32).collect();
    println!("{}", _a * _b[2]);
    println!("{}", _a * _b[1]);
    println!("{}", _a * _b[0]);
    println!("{}", _a * b.trim().parse::<i32>().unwrap());
}
