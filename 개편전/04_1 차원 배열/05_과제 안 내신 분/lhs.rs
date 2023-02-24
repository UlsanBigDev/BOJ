use std;

fn main(){
    let mut buf = String::new();
    let stdin = std::io::stdin();
    
    let mut list : Vec<u8> = Vec::new();
    let mut result : Vec<u8> = Vec::new();

    for _ in 0..28{
        stdin.read_line(&mut buf).err();
        let temp = buf.clone().trim().parse::<u8>().unwrap();
        list.push(temp);
        buf.clear();
    }
    for i in 1..=30{
        if (!list.contains(&i)){
            result.push(i);
        }
    }

    println!("{}", result.iter().min().unwrap());
    print!("{}", result.iter().max().unwrap());

}
