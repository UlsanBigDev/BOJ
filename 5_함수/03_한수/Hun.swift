var n = Int(readLine()!)!
var sum = 0

for i in 1...n {
    if i == 1000 {
        break
    }
    
    if i > 99 {
        var arr = String(i).map{Int(String($0))!}
        
        if (arr[2] - arr[1]) == (arr[1] - arr[0]) {
            sum += 1
        }
    } else {
        sum += 1
    }
}

print(sum)
