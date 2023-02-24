let arr = readLine()!.split(separator: " ")
let A = Int(String(arr[0].reversed()))!
let B = Int(String(arr[1].reversed()))!

A > B ? print(A) : print(B)
