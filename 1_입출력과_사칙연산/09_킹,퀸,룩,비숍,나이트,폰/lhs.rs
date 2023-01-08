fn main(){
    let mut a: String = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut a);

    let b:Vec<i32> = a.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
    println!("{} {} {} {} {} {}", 1 - b[0], 1 - b[1], 2 - b[2], 2 - b[3], 2 - b[4], 8 - b[5]);
}
