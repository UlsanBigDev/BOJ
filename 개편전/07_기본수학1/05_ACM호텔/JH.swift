let T = Int(readLine()!)!

for _ in 0..<T{
    let arr = readLine()!.split(separator: " ").map{Int(String($0))!}
    if arr[2] % arr[0] == 0 {
        print(arr[0]*100 + (arr[2]/arr[0]))
    }
    else {
        print((arr[2] % arr[0])*100 + (arr[2]/arr[0]) + 1)
    }
}
