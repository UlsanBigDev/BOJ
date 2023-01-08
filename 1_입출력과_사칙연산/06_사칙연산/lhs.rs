//mem : 16304 KB | time : 4 ms
fn main(){
    let mut a: String = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut a);

    let result:Vec<f64> = a.trim()
        .split_whitespace()
        .map(|x| x.parse::<f64>().unwrap())
        .collect();
    
    println!("{}", result[0] + result[1]);
    println!("{}", result[0] - result[1]);
    println!("{}", result[0] * result[1]);
    println!("{}", (result[0] / result[1]) as i32);
    println!("{}", result[0] % result[1]);
}
