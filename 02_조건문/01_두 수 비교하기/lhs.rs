fn main(){
    let mut text :String = String::new();

    let stdin = std::io::stdin();
    stdin.read_line(&mut text).unwrap();
    let b :Vec<i32> =text.trim().split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    
    if b[0] > b[1]{
        println!(">");
    }else if b[0] < b[1]{
        println!("<");
    }else{
        println!("==");
    }
}
