fn main(){
    let mut buf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buf).unwrap();

    let t = buf.trim().split_whitespace().map(|v| v.parse::<f64>().unwrap()).collect::<Vec<f64>>();

    let (a, b, v) = (t[0], t[1], t[2]);

    let result = (((v - a)/ (a - b)) as f64).ceil() + 1_f64;
    print!("{result}");
}
