fn main(){
    let mut buf = String::new();
    let stdin = std::io::stdin();
    
    let mut buf_str = String::new();
    buf.clear();

    loop{
        stdin.read_line(&mut buf).unwrap();
        if buf.trim() == "" {
            break;
        }
        buf_str.push_str(buf.clone().as_str());
        buf.clear();
    }
    print!("{buf_str}");
}
