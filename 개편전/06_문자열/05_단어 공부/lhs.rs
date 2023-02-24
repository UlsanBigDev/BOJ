fn main() {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buf).unwrap();

    let text  = buf.trim().to_string();
    let text = text.to_lowercase();

    let mut alpha = vec![0; 26];

    let text = text.bytes().map(|v| v - 0x61).collect::<Vec<u8>>();

    for (_, t) in text.iter().enumerate(){
        alpha[*t as usize] += 1;
    }

    if alpha.iter().filter(|&&n| n == *alpha.iter().max().unwrap()).count() > 1{
        print!("?");
    }else{
        let pos = alpha.iter().position(|v| *v == *alpha.iter().max().unwrap()).unwrap();
        print!("{}", (pos as u8 + 'A' as u8 ) as char);
    }
}
