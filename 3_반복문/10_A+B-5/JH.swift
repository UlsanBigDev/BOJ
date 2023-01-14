import Foundation

while true {
    let num = readLine()!.components(separatedBy: " ").map{Int(String($0))!}
    if num[0]==0 && num[1]==0{
        break
    }
    print(num[0] + num[1])
}
