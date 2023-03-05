fn main(){
    let mut buf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buf).unwrap();

    let v = buf.clone().split_whitespace().map(|v| v.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let len = v[0];
    let count = v[1];
    buf.clear();

    let mut list  : Vec<i32>= vec![0; len as usize];
    for _ in 0..count{
        stdin.read_line(&mut buf).unwrap();
        let mut v = buf.clone().split_whitespace().map(|v| v.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        for i in (v[0]-1)..v[1]{
            list[i] = v[2] as i32;
        }
        buf.clear();
    }
    
    for i in 0..len{
        print!("{} ", list[i as usize]);
    }
}
