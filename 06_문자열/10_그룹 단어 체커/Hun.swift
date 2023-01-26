let input = Int(readLine()!)!
var answer = 0

for _ in 1...input {
    var str = Array(readLine()!)
    var arr:[Character] = []
    
    for s in str {
        if !arr.contains(s) {
            arr.append(s)
        } else if arr.last! != s {
            arr.removeAll()
            break
        }
    }
    
    if Set(str).count == arr.count { answer += 1}
}
print(answer)
