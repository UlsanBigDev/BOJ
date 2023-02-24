fn main(){
    let mut buf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buf).unwrap();

    let n = buf.clone().trim().parse::<u16>().unwrap();

    
    buf.clear();
    for _ in 0..n{
        stdin.read_line(&mut buf).unwrap();
        let line : Vec<u32>= buf.clone().split_whitespace().map(|v| v.parse::<u32>().unwrap()).collect();
        let size = line[0];
        let score = &line[1..];
        
        let average = (score.iter().sum::<u32>() / size) as f64;

        let over_num = score.iter().filter(|v| **v > average as u32).count();
        
        println!("{:.3}%",over_num as f64 * 100_f64 / size as f64);
        buf.clear();
        
    }
    
}
