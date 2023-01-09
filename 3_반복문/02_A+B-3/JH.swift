import Foundation

let a = Int(readLine()!)!

for i in 0..<a{
    let arr = readLine()!.split(separator: " ").map{Int($0)!}
    print(arr[0]+arr[1])
}
