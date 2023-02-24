import Foundation

var loop = Int(readLine()!)!

for _ in 1...loop {
    var input = readLine()!
    var arr = input.components(separatedBy: " ").map{Int($0)!}

    print(arr[0] + arr[1])
}
