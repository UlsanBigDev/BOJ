import Foundation
var a=readLine()!
var arr=a.components(separatedBy: " ")
var H=Int(arr[0])!
var M=Int(arr[1])!

if M>=45{
    M=M-45
}else{
    M=M-45+60
    if H==0{
        H=23
    }else{
        H=H-1
    }
}
print("\(H) \(M)")
