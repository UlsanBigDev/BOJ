import Foundation

let a = Int(readLine()!)!
let b = Int(readLine()!)!
var hap: Int = 0

for _ in 0..<b{
    let arr = readLine()!.split(separator: " ").map{Int($0)!}
    hap+=arr[0]*arr[1]
}

print(hap==a ? "Yes" : "No")
