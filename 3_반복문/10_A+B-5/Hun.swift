while true {
    let num = readLine()!.split(separator:" ").map{Int($0)!}

    if num == [0,0] { break }
    print(num[0] + num[1])
}
