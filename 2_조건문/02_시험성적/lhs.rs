fn main(){
    let mut text :String = String::new();

    let stdin = std::io::stdin();
    stdin.read_line(&mut text).unwrap();
    let b = text.trim().parse::<i32>().unwrap();
    
    if b >= 90{
        println!("A");
    }else if b >= 80{
        println!("B");
    }else if b >= 70{
        println!("C");
    }else if b >= 60{
        println!("D");
    }else{
        println!("F");
    }
}
