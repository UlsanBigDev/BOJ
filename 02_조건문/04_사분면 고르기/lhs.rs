fn main(){
    let mut a : String = String::new();
    let mut b : String = String::new();

    let stdin = std::io::stdin();
    stdin.read_line(&mut a).unwrap();
    stdin.read_line(&mut b).unwrap();
    
    let a = a.trim().parse::<i32>().unwrap();
    let b = b.trim().parse::<i32>().unwrap();

    if a > 0 && b > 0 {
        println!("1");
    }else if a < 0 && b > 0{
        println!("2");
    }
    else if a < 0 && b < 0{
        println!("3");
    }else{
        println!("4");
    }
}
