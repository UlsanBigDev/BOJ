var s = Array(readLine()!)
var arr: [Int] = []

for i in 97...122 {
    let alpha = Character(UnicodeScalar(i)!)
    
    if s.contains(alpha) {
        arr.append(s.firstIndex(of: alpha)!)
    }else {
        arr.append(-1)
    }
}
print(arr.map{String($0)}.joined(separator: " "))
