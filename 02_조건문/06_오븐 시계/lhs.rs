fn main(){
    let mut text : String = String::new();

    let stdin = std::io::stdin();
    stdin.read_line(&mut text).unwrap();
    let text:Vec<i32> = text.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
    
    let h = text[0];
    let m = text[1];

    let mut b = String::new();
    stdin.read_line(&mut b);
    let b: i32 = b.trim().parse().unwrap();

    let _h = (b as f64 / 60.0) as i32;
    let _m = b % 60;
    println!("{} {}", ((h + _h) + (m + _m)/60)%24, (m + _m)%60);
}
