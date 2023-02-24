let T = Int(readLine()!)!

for _ in 0..<T{
    let arr = readLine()!.split(separator: " ")
    let R = Int(arr[0])!
    let S = Array(arr[1])
    for i in 0..<S.count{
        for _ in 0..<R{
            print(S[i], terminator: "")
        }
    }
    print()
}
