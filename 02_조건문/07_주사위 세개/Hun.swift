import Foundation

var input = readLine()!
var arr = input.components(separatedBy: " ").map{Int($0)!}
var money = 0

if Set(arr).count == 1 {
    money = 10000 + arr[0] * 1000
}else if Set(arr).count == 2 {
    money = 1000 + (arr.filter{$0 == arr[0]}.count == 2 ? arr[0] * 100 : arr[1] * 100)
}else{
    money = arr.max()! * 100
}
print(money)
