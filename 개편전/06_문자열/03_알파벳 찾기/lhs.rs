fn main() {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buf).unwrap();

    let text  = buf.trim().to_string();

    let mut alpha = vec![-1; 26];

    let text = text.bytes().map(|v| v - 0x61).collect::<Vec<u8>>();

    for i in 0..26_u8{
        alpha[i as usize] = match text.iter().position(|&x| x == i){
            None => -1,
            Some(v) => v as i32,
        };
    }

    for (_, t) in alpha.iter().enumerate(){
        print!("{t} ");
    }
}
