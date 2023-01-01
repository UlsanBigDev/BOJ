import Foundation

var a = readLine()!
var sum = a.components(separatedBy: " ").map{Int($0)!}

print(sum[0] + sum[1])
print(sum[0] - sum[1])
print(sum[0] * sum[1])
print(sum[0] / sum[1])
print(sum[0] % sum[1])
