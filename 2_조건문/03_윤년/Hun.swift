import Foundation

var year = Int(readLine()!)!
print(year % 4 == 0 ? year % 100 != 0 ? "1" : year % 400 == 0 ?  "1" : "0" : "0")
