import Foundation

import Foundation

var a = readLine()!
var sum = a.components(separatedBy: " ").map{Int($0)!}.reduce(0,+)

print(sum)
