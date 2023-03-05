let input = readLine()!.split(separator: " ").map{Int(String($0))!}
var arr = Array(repeating: 0, count: input[0])
let loop = input[1]


for _ in 0..<loop{
    let input2 = readLine()!.split(separator: " ").map{Int(String($0))!}
    let start = input2[0] - 1
    let end = input2[1] - 1
    let num = input2[2]
    for i in start...end{
        arr[i] = num
    }
}

for i in arr{
    print(i, terminator: " ")
}
