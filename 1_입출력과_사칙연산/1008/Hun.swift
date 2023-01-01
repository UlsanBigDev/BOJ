import Foundation

var a = readLine()!
var sum = a.components(separatedBy: " ").map{Double($0)!}

print(sum[0] / sum[1])
