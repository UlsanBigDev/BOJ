var arraySet : Set<Int> = Set<Int>()
for _ in 1...10{
    let A = Int(readLine()!)!
    arraySet.insert(A % 42)
}

print(arraySet.count)
