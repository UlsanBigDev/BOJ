fn main(){
    let mut b : String = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut b).unwrap();
    let b = b.trim().parse::<i32>().unwrap();
    if b % 4 == 0 && (b % 100 != 0 || b % 400 == 0){
        println!("1");
    }else{
        println!("0");
    }
}
