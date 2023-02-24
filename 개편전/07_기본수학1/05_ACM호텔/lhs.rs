fn main(){
    let mut buf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buf).unwrap();

    let mut n = buf.clone().trim().parse::<i32>().unwrap();
    buf.clear();

    for _ in 0..n{
        stdin.read_line(&mut buf).unwrap();
        let t = buf.trim().split_whitespace().map(|v| v.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let (h, w, v) = (t[0], t[1], t[2]);

        let mut result = 0;
        
        if v % h == 0{
            result += h * 100;
        }else{
            result += (v % h) * 100;
        }

        result += ((v-1) / h) as i32 + 1;
        println!("{result}");
        buf.clear();
    }
}
