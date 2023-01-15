var input = Int(readLine()!)!
let firstNum = input
var temp = 0
var n = 0

repeat {
    temp = ((input / 10) + (input % 10)) % 10 + (input % 10) * 10
    input = temp
    n += 1
} while firstNum != temp

print(n)
