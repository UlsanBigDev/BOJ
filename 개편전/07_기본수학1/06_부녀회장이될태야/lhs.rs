fn sum(a : i32, b : i32) -> i32{ // 1부터 n까지의 합
    if a == 0{
        b
    }else if b == 1{
        1
    }else{
        sum(a-1, b) + sum(a, b-1)
    }

}

fn main(){
    let mut buf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buf).unwrap();

    let n = buf.clone().trim().parse::<i32>().unwrap();

    buf.clear();
    let mut v : Vec<i32> = Vec::new();
    for _ in 0..n{
        
        for _ in 0..2{
            stdin.read_line(&mut buf).unwrap();
            v.push(buf.trim().parse::<i32>().unwrap());
            buf.clear();
        }
        let a = v[0];
        let b = v[1];
        println!("{}", sum(a, b));
        v.clear();
    }

}
