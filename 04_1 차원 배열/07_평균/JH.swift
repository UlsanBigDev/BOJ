import Foundation

let A = Int(readLine()!)!
let arr = readLine()!.split(separator: " ").map{Int($0)!}
let avg = Float(arr.reduce(0, +))/Float(A)
var max = 0

for i in 0..<A where Int(max) < arr[i]{
    max = arr[i]
}

print(Float(avg)/Float(max) * 100)
