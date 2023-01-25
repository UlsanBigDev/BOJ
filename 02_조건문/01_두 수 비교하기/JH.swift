import Foundation
var a=readLine()!
var arr=a.components(separatedBy: " ")

let A=Int(arr[0])!
let B=Int(arr[1])!

if A>B{
    print(">")
}else if A<B{
    print("<")
}else if A==B{
    print("==")
}
