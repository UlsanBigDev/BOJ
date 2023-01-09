use std::{self, io::{Write, BufWriter}};

fn main(){
    let mut buf = String::new();
    let stdin = std::io::stdin();

    stdin.read_line(&mut buf).unwrap();
    let n : i32 = buf.trim().parse().unwrap();

    let stdout = std::io::stdout();
    let mut br = BufWriter::new(stdout.lock());
    
    for i in 0..n{
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        let ans = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        writeln!(br, "Case #{}: {} + {} = {}",i+1, ans[0], ans[1], ans[0] + ans[1]).unwrap();
    }

}
