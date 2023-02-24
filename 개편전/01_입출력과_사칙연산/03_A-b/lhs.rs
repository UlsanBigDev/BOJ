//mem : 13512 KB | time : 4 ms
fn main(){
    let mut a: String = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut a);

    let result:Vec<i32> = a.trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    
    
    println!("{}",result[0] - result[1]);
}
