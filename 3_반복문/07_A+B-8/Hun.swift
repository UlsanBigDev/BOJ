import Foundation

var loop = Int(readLine()!)!

for i in 1...loop {
    let arr = readLine()!.components(separatedBy: " ").map{Int($0)!}

    print("Case #\(i): \(arr[0]) + \(arr[1]) = \(arr[0] + arr[1])")
}
