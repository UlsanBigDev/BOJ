import Foundation

let condition = readLine()!.components(separatedBy: " ").map{Int($0)!}
let n = readLine()!.components(separatedBy: " ").map{Int($0)!}.filter{$0 < condition[1]}.map{String($0)}.joined(separator: " ")
print(n)
