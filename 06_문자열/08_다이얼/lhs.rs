fn main() {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buf).unwrap();

    let text  = buf.trim().bytes().collect::<Vec<u8>>();


    let alpha = ["ABC", "DEF", "GHI", "JKL", "MNO", "PQRS", "TUV", "WXYZ"];
    let alpha = alpha.map(|v| v.bytes().collect::<Vec<u8>>());

    let mut result = 0;

    for (_, c) in text.iter().enumerate(){
        for i in 0..alpha.len(){
            if alpha[i].contains(c){
                result += i;
            }
        }
    }
    result += text.len() * 3;
    println!("{:?}", result);
}
