use std;

fn main(){
    let mut buf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buf).unwrap();

    let mut stack = 0_u32;
    let mut result = 0_u32;

    let n = buf.clone().trim().parse::<u8>().unwrap();
    buf.clear();
    for _ in 0..n{
        stdin.read_line(&mut buf).unwrap();
        let n = buf.clone().trim().to_string();
        for i in n.bytes(){
            if(i == b'O'){
                stack += 1;
                result += stack;
            }else{
                stack = 0;
            }
        }
        println!("{}", result);
        stack = 0;
        result = 0;
        buf.clear();
    }
}
