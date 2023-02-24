use std;

fn main(){

    let mut buf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buf).unwrap();

    let buf = buf.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let x = buf[1];

    let mut seq = String::new();
    stdin.read_line(&mut seq).unwrap();
    let seq = seq.split_whitespace()
                            .filter(|&y| y.parse::<i32>().unwrap() < x)
                            .map(|x| String::from(x.to_owned() + " "))
                            .collect::<String>();
    print!("{}", seq);
}
