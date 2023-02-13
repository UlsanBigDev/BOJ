import Foundation

let arr = ["c=","c-","dz=","d-","lj","nj","s=","z="]
var str = readLine()!
for i in 0..<arr.count{
    str = str.replacingOccurrences(of: arr[i], with: "a")
}
print(str.count)
