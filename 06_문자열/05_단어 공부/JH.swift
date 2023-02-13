let str = readLine()!.uppercased().map{String($0)}
var arr = Array(repeating: 0, count: 26)
for i in str{
    let num = Int(UnicodeScalar(i)!.value-65)
    arr[num] += 1
}

arr.filter{arr.max() ?? 0 == $0}.count >= 2 ? print("?") : print(String(UnicodeScalar(65 + Int(arr.firstIndex(of: arr.max() ?? 0) ?? 0))!))
