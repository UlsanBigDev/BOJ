let n = Int(readLine()!)!

for _ in 0..<n {
    var sum = 0
    var quiz = readLine()!.split(separator: "X").map{$0}

    for s in quiz {
        sum += (1...s.count).reduce(0,+)
    }
    print(sum)
}
