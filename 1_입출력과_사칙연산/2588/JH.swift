import Foundation

var A=Int(readLine()!)!
var B=Int(readLine()!)!

let C=Int(B%10/1)
let D=Int(B%100/10)
let E=Int(B/100)

print(A*C)
print(A*D)
print(A*E)
print(A*B)
