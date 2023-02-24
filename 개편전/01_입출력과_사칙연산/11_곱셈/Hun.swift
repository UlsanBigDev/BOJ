import Foundation

var a = Int(readLine()!)!
var b = readLine()!
var arrB = b.map{Int(String($0))!}

print(a * arrB[2])
print(a * arrB[1])
print(a * arrB[0])
print(a * Int(b)!)
