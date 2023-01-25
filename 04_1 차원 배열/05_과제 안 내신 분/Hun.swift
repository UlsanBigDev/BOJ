var arr = [2] + [Int](repeating: 0, count: 30)

for _ in 1...28 {
    let temp = Int(readLine()!)!

    arr[temp] = 1
}
print("\(arr.firstIndex(of: 0)!)\n\(arr.lastIndex(of: 0)!)")


// Set 사용
var temp = Set<Int>()

for _ in 1...28 {
    temp.insert(Int(readLine()!)!)
}

for i in 1...30 {
    if !temp.contains(i) {
        print(i)
    }
}
