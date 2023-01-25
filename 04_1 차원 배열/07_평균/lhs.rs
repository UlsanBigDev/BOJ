use std;

fn main(){
    let stdin = std::io::stdin();
    let mut n = String::new();

    stdin.read_line(&mut n).unwrap();
    let n = n.trim().parse::<i32>().unwrap();
    
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    let mut vec = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let max = *vec.iter().max().unwrap();

    let sum = vec.iter().map(|&x| ((x as f64) * 100.0 / (max as f64)) as f64).collect::<Vec<f64>>().iter().sum::<f64>();
    println!("{:.6}", sum/ (n as f64));
}
