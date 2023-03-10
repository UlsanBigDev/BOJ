use std;

fn main(){
    let stdin = std::io::stdin();
    let mut vec :Vec<u32>= Vec::new();
    for _ in 0..10{
        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();
        let buf = buf.trim().parse::<u32>().unwrap() % 42;

        if !vec.contains(&buf){
            vec.push(buf);
        }
    }
    println!("{}", vec.len());
}

// bit masking

use std;

fn main(){
    let stdin = std::io::stdin();
    
    let mut vec = [false; 43];
    for _ in 0..10{
        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();
        let buf = buf.trim().parse::<u32>().unwrap() % 42;
        
        vec[buf as usize] |= true; //or 후 대입
    }

    println!("{}", vec.iter().filter(|&n| *n == true).count());
}
