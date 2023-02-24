import Foundation

let N = Int(readLine()!)!

for _ in 0..<N{
    var sum = 0
    var score = 0
    let arr = Array(readLine()!)
    
    for i in 0..<arr.count{
        if arr[i]=="O"{
            score += 1
            sum += score
        }
        else{
            score = 0
        }
    }
    print(sum)
}
