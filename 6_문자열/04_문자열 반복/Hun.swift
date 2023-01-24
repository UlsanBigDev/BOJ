let i = Int(readLine()!)!

for _ in 1...i {
    var arr = readLine()!.split(separator: " ")
    print(arr[1].map{String(repeating: $0, count: Int(arr[0])!)}.joined())
}
