var a = [Int]()
for _ in 0..<9{
    let N = Int(readLine()!)!
    a.append(N)
}
print(a.max()!)
print(a.index(of: a.max()!)!+1)
