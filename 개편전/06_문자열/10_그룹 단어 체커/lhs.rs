fn main() {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    
    stdin.read_line(&mut buf).unwrap();
    let n = buf.clone().trim().parse::<i32>().unwrap();

    let mut stack : Vec<char> = Vec::new();
    let mut ex = 0;

    buf.clear();
    for _ in 0..n{
        stdin.read_line(&mut buf).unwrap();
        let text  = buf.trim().chars();
        for (_, t) in text.collect::<Vec<char>>().iter().enumerate(){
            
            if stack.is_empty(){
                stack.push(*t);
            }            

            if stack.last().unwrap() != t {
                if stack.contains(t){
                    ex += 1;
    
                    stack.clear();
                    buf.clear();
                    break;
                }
                stack.push(*t);
            }
        }
        stack.clear();
        buf.clear();
    }
    print!("{}", n - ex);
    
}
