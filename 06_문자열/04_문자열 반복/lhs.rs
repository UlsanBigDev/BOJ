fn main() {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buf).unwrap();

    let n = buf.clone().trim().parse::<i32>().unwrap();
    buf.clear();

    for _ in 0..n{
        stdin.read_line(&mut buf).unwrap();
        let mut wh = buf.trim().split_whitespace();

        let count = wh.next().unwrap();
        let text = wh.next().unwrap();

        let text = text.chars().collect::<Vec<char>>();
        for i in 0..text.len(){
            for _ in 0..count.parse::<usize>().unwrap(){
                print!("{}", text[i]);
            }
        }
        println!("");
        buf.clear();
    }
}
