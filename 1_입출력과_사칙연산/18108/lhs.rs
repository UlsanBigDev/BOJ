//mem : 13148 KB | time : 4 ms
fn main(){

    let mut a: String = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut a);
    
    println!("{}", a.trim().parse::<i32>().unwrap() - 543);
}
