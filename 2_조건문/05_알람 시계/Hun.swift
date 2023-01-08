import Foundation

var input = readLine()!
var arr = input.components(separatedBy: " ").map{Int($0)!}
var H = arr[0]
var M = arr[1]

if M < 45 {
    if H == 0 {
        H += 23
        M += 15
    } else {
        H -= 1
        M += 15
    }
} else {
    M -= 45
}

print(H, M)
