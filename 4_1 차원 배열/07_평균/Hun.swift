let n = Double(readLine()!)!
let arr = readLine()!.split(separator: " ").map{Double($0)!}

print(arr.reduce(0,+) / Double(arr.max()!) * 100.0 / n)
