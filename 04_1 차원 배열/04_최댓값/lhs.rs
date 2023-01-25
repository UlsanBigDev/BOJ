use std;

fn main(){
    let stdin = std::io::stdin();
    let mut max = 0;
    let mut index = 1;

    for i in 1..=9{
        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();
        let buf = buf.trim().parse::<i32>().unwrap();
        max = if buf > max{
            index = i;
            buf
        }else{
            max
        };
    }

    println!("{}", max);
    print!("{}", index);
}
