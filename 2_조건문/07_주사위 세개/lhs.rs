fn main(){
    let mut text = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut text).unwrap();

    let dices :Vec<i32> = text.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();

    let result = if dices[0] == dices[1] && dices[1] == dices[2] {
        10000 + dices[0] * 1000
    }else if dices[0] == dices[1]{
        1000 + dices[0] * 100
    }else if dices[1] == dices[2]{
        1000 + dices[1] * 100
    }else if dices[0] == dices[2]{
        1000 + dices[0] * 100
    }else{
        *dices.iter().max().unwrap() as i32 * 100
    };

    println!("{}", result);
}
