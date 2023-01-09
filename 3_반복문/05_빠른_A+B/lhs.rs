use std::{self, io::{Write, BufWriter}};

fn main(){
    let mut n = String::new();
    let stdin = std::io::stdin();
    
    stdin.read_line(&mut n).unwrap();

    let n = n.trim().parse::<i32>().unwrap();

    let stdout = std::io::stdout();
    let mut test = BufWriter::new(stdout.lock()); 
    // 기존 출력 println!(); 는 입력을 받고 한 줄 마다 버퍼를 비우기(flush) 때문에 오버헤드가 큼. 
    
    for _ in 0..n{
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        let sum:i32 = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).sum();
        
        writeln!(test, "{}", sum).unwrap(); // 따라서 입력을 버퍼에 한번에 저장한 후,
    }
    //입력이 끝나면 전부 출력
    
}
