import Foundation

var a = readLine()!
var arr = a.components(separatedBy:" ").map{Int($0)!}
var A = arr[0]
var B = arr[1]
var C = arr[2]

print((A+B)%C)
print(((A%C) + (B%C))%C)
print((A*B)%C)
print(((A%C) * (B%C))%C)
