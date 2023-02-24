var max = 0
var count = 0
var temp = 0

for i in 1...9 {
    temp = Int(readLine()!)!

    if temp > max {
        max = temp
        count = i
    }
}

print("\(max)\n\(count)")
