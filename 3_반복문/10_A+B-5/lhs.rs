use std::{self, io::{BufWriter,Write}};

fn main(){
    let mut buf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buf).unwrap();
    let mut buf : Vec<i32> = buf.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();

    let stdout = std::io::stdout();
    let mut br = BufWriter::new(stdout.lock());

    while !(buf[0] == 0 && buf[1] == 0){
        writeln!(br, "{}", buf[0] + buf[1]).unwrap();
        buf.clear();
        let mut task = String::new();
        stdin.read_line(&mut task).unwrap();
        buf = task.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
    }
}
