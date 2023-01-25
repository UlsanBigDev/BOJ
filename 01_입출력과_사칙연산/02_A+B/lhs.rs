//mem : 13152 KB | time : 4 ms
fn main(){
    let mut a: String = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut a);

    let result = a.trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .sum::<i32>();
    
    println!("{}",result);
}
