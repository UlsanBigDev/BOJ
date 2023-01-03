//mem : 13195 KB | time : 4 ms
fn main(){
    let mut a: String = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut a);

    let result:Vec<f64> = a.trim()
        .split_whitespace()
        .map(|x| x.parse::<f64>().unwrap())
        .collect();
    
    
    println!("{:.9}", result[0] / result[1]);
}
