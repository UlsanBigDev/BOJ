import Foundation

let n = Int(readLine()!)!

for _ in 0..<n {
    var arr = readLine()!.split(separator: " ").map{Int($0)!}
    var len = arr[0]
    arr.remove(at: 0)

    var above = arr.filter{ $0 > arr.reduce(0,+) / len}.count

    print(String(format: "%.3f", Float(above) / Float(len) * 100) + "%")
}
