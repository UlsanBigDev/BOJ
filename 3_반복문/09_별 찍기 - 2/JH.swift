import Foundation
let N = Int(readLine()!)!
var star : String = ""
var blank : String

for i in 1...N{
    blank = String(repeating: " ", count: N - i)
    star += "*"
    print(blank + star)
}
