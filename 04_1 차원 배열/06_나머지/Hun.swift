var temp = Set<Int>()

for _ in 1...10 {
    temp.insert(Int(readLine()!)! % 42)
}
print(temp.count)
