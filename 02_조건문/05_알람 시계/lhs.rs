fn main(){
    let mut text : String = String::new();

    let stdin = std::io::stdin();
    stdin.read_line(&mut text).unwrap();
    let text:Vec<i32> = text.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
    let h = text[0];
    let m = text[1];
    println!("{} {}", if m < 45 {(h+23)%24}else{h}, (m + 15)%60);
}
