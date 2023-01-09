import Foundation

let T = Int(readLine()!)!

for i in 0..<T {
    let arr = readLine()!.components(separatedBy: " ").map{Int($0)!}
    print("Case #\(i+1): \(arr[0]) + \(arr[1]) = \(arr[0]+arr[1])")
}
