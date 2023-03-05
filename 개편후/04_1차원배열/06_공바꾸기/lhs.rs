fn main(){
    let mut buf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buf).unwrap();
    
    let v = buf.clone().trim().split_whitespace().map(|v|v.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let (n, m) = (v[0], v[1]);
    let mut list = (1..=n).collect::<Vec<i32>>();

    buf.clear();

    for _ in 0..m {
        stdin.read_line(&mut buf).unwrap();
        let v = buf.clone().trim().split_whitespace().map(|v|v.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let (i, j) : (i32, i32)= (v[0], v[1]);
        list.swap((i-1) as usize, (j-1) as usize);
        buf.clear();
    }

    for i in 0..n{
        print!("{} ", list[i as usize]);
    }
}
