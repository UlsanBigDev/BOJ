import Foundation

var a = readLine()!
var arr = a.components(separatedBy: " ").map{Int($0)!}

print(1 - arr[0], 1 - arr[1], 2 - arr[2], 2 - arr[3], 2 - arr[4], 8 - arr[5])
