import Foundation
let arr1 = readLine()!.components(separatedBy: " ").map{Int(String($0))!}
let N = arr1[0]
let X = arr1[1]
let arr2 = readLine()!.components(separatedBy: " ").map{Int(String($0))!}
var numbers = [String]()

for i in 0..<N {
    if arr2[i] < X {
        numbers.append(String(arr2[i]))
    }
}
print(numbers.joined(separator: " "))
