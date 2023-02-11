fn main() {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buf).unwrap();

    let text  = buf.trim().to_string();

    let mut ex_len : i32 = 0;

    let croatian = ["c=", "c-", "dz=", "d-", "lj", "nj", "s=", "z="];

    let mut result : i32 = 0;

    for (_, c) in croatian.iter().enumerate(){

        if text.matches(c).count() > 0 && c == &"dz="{
            result += text.matches(c).count() as i32;
        }
        
        result += text.matches(c).count() as i32;
        ex_len += c.len() as i32 * text.matches(c).count() as i32;
    }
    
    print!("{}", text.len() as i32 - ex_len + result);
}
