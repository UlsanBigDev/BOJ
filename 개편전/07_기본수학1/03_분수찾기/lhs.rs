fn main() {
    let mut buf = String::new();
    let stdin = std::io::stdin();

    let mut i = 1;

    stdin.read_line(&mut buf).unwrap();
    let n = buf.trim().parse::<i32>().unwrap();
    
    loop {
        if n <= i * (i-1) / 2{
            break;
        }
        i+=1;
    }

    if i % 2 == 1{ // 1부터
        let count = n - (i-1) * (i-2) / 2; 
        println!("{}/{}",count, i-count);
    }else{ // i부터
        let count = n - (i-1) * (i-2) / 2; 
        println!("{}/{}", i-count,count)
    }
    
}
