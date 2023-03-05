let T = Int(readLine()!)!

for _ in 0..<T{
    let str = readLine()!.map{String($0)}
    print("\(str[0])\(str[str.count-1])")
}
