import Foundation
var a=readLine()!
var arr=a.components(separatedBy:" ")
let A=Int(arr[0])!
let B=Int(arr[1])!
let C=Int(arr[2])!
print((A+B)%C)
print(((A%C) + (B%C))%C)
print((A*B)%C)
print(((A%C)*(B%C))%C)
