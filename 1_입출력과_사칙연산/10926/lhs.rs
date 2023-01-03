//mem : 13152 KB | time : 4 ms
fn main(){

    let mut a: String = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut a);

    let mut text = a.trim().to_owned().to_string();
    text.push_str(&format!("{}", "??!"));

    println!("{}", text);
}
