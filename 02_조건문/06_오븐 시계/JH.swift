import Foundation
var a = readLine()!
var plusM = Int(readLine()!)!
var arr = a.components(separatedBy: " ")
var H = Int(arr[0])!
var M = Int(arr[1])!

M = M + plusM

if M >= 60{
    H = H + (M / 60)
    M = M % 60
    if H>=24{
        H = H - 24
    }
}

print("\(H) \(M)")
