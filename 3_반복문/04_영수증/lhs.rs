use std;

fn main(){
    let mut total = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut total).unwrap(); // total 입력 받기

    let mut count = String::new();
    stdin.read_line(&mut count).unwrap();
    let count = count.trim().parse::<i32>().unwrap(); // count 입력 받기
    
    let mut container = 0; // 임시 변수

    for _ in 0..count{
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        let b = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>(); // count번 입력 받음
      
        container = container + b[0] * b[1]; // 두 수의 곱의 결과를 임시 변수에 더함
    }
    let total = total.trim().parse::<i32>().unwrap();

    let result = if total == container{
        "Yes"
    }else{
        "No"
    };
    
    print!("{}", result);
}
