use std;

fn main(){
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse::<usize>().unwrap();

    for i in (0..n).rev(){ 
        println!("{:*>n$}", " ".repeat(i));
    }
}
