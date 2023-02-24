import Foundation

var price = Int(readLine()!)!
let num = Int(readLine()!)!

for _ in 1...num {
    var input = readLine()!
    var arr = input.components(separatedBy: " ").map{Int($0)!}

    price -= arr[0] * arr[1]
}

print(price == 0 ? "Yes" : "No")
