import Foundation
let N = Int(readLine()!)!
let arr = readLine()!.components(separatedBy: " ").map{Int(String($0))!}
let v = Int(readLine()!)!
var count : Int = 0

for i in 0..<N {
    if arr[i] == v {
        count += 1
    }
}
print(count)
