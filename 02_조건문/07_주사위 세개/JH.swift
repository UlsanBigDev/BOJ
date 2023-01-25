import Foundation
var a = readLine()!
var arr = a.components(separatedBy: " ")
let A = Int(arr[0])!
let B = Int(arr[1])!
let C = Int(arr[2])!
let reward: Int
let Max: Int

if A==B && B==C{
    reward = Int(10000 + A*1000)
}else if A==B || A==C{
    reward = Int(1000 + A*100)
}else if B==C || B==A{
    reward = Int(1000 + B*100)
}else{
    Max = max(A, B, C)
    reward = Int(Max * 100)
    }


print(reward)
