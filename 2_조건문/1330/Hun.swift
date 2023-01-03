import Foundation

var a = readLine()!
var arr = a.components(separatedBy: " ").map{Int($0)!}

print(arr[0] == arr[1] ? "==" : arr[0] > arr[1] ? ">" : "<")
