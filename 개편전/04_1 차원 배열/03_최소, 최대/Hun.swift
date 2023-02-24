// 속도 -> components(separatedBy:) < split(separator: " ")

let n: Int = Int(readLine()!)!
let arr:[Int] = readLine()!.split(separator: " ").map{Int($0)!}
print(arr.min()!, arr.max()!)
