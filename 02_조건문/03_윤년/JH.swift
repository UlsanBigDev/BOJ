import Foundation
let a=Int(readLine()!)!
let A=a%4
let B=a%100
let C=a%400

if A==0{
    print(!(B==0) || C==0 ? "1" : "0")
}else{
    print("0")
}
