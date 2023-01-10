use std::io;

fn main() {
    let mut arr: [bool; 30] = [false; 30];
    for _ in 0..28 {
        let mut a = String::new();
        io::stdin().read_line(&mut a).unwrap();
        let a: usize = a.trim().parse().unwrap();
        arr[a - 1] = true;
    }

    for (index, value) in arr.iter().enumerate() {
        if !value {
            println!("{}", index + 1);
        }
    }
}
