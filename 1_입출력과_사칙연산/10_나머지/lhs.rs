fn main(){
    let mut a: String = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut a);

    let b:Vec<i32> = a.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
    println!("{} {} {} {}", (b[0] + b[1])%b[2], ((b[0]%b[2]) + b[1]%b[2])%b[2], (b[0] * b[1])%b[2], (b[0]%b[2] * b[1]%b[2])%b[2]);
}
