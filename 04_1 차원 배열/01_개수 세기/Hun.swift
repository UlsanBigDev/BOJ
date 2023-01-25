import Foundation

var n = Int(readLine()!)!
var input = readLine()!.components(separatedBy: " ").map{Int($0)!}
var v = Int(readLine()!)!

print(input.filter{$0 == v}.count)
