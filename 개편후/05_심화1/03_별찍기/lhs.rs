fn main(){
    let mut buf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buf).unwrap();

    let n = buf.trim().parse::<i32>().unwrap();

    let stars = vec!['*'; (n*2-1) as usize];
    let blanks = vec![' '; (n-1) as usize];

    for i in 0..(n * 2 - 1){
        let t = (i - (n-1)).abs();
        println!("{}{}", blanks[..t as usize].into_iter().collect::<String>(), stars[..((n * 2 - 1) - 2 * t) as usize].into_iter().collect::<String>());
    }
}
