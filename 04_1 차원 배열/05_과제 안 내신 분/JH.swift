var X = [Int]()
for _ in 0..<28{
    let N = Int(readLine()!)!
    X.append(N)
}
for i in 1...30{
    if X.contains(i){
        X.remove(at: X.firstIndex(of : i)!)
    }
    else {
        X.append(i)
    }
}

print(X.min()!)
print(X.max()!)
