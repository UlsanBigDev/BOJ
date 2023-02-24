use std;

fn main(){
    let mut buf = String::new();

    let stdin = std::io::stdin();
    stdin.read_line(&mut buf).unwrap();
    let mut buf = buf.trim().parse::<u64>().unwrap();
    let start = buf;

    let mut i = 0_i64;
    loop{
        if buf <10{
            buf = buf * 10 + buf;
            i += 1;
        }else{
            buf = (buf % 10) * 10 + ((buf % 10) + (buf / 10) as u64) % 10;
            i += 1;
        }

        if buf == start{
            print!("{}", i);
            break;
        }
    }
    
}
