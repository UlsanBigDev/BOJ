import Foundation

var input = readLine()!
var time = Int(readLine()!)!
var arr = input.components(separatedBy: " ").map{Int($0)!}
var H = arr[0]
var M = arr[1]

if M + time >= 60{
    H += (M + time) / 60
    M = (M + time) % 60
    
    if H > 23 {
        H -= 24
    }
}else {
    M += time
}

print(H, M)
