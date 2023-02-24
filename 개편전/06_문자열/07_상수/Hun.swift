var arr = readLine()!.split(separator: " ")
var first = Int(String(arr[0].reversed()))!
var second = Int(String(arr[1].reversed()))!

print(first > second ? first : second)
