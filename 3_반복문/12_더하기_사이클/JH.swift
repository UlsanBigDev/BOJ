import Foundation

var Front : Int
var Back : Int
var New : Int
var Count : Int = 0
let A = Int(readLine()!)!
var AA : Int = A

while true {
    Front = Int(AA / 10)
    Back = Int(AA % 10)
    New = Back * 10 + (Front + Back) % 10
    Count = Count + 1
    if A == New {
        break
    }
    AA = New
}
print(Count)
