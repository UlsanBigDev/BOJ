import Foundation

let C = Int(readLine()!)!

for _ in 0..<C{
    var avg = 0
    var count = 0
    let arr = readLine()!.split(separator: " ").map{Int(String($0))!}
    
    avg = (arr.reduce(0, +)-arr[0])/arr[0]
    for i in 1..<arr.count{
        if arr[i] > avg{
            count += 1
        }
    }
    print("\(String(format: "%.3f", Float(count)/Float(arr.count-1)*100))%")
}
