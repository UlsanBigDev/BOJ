print(readLine()!.split(separator: " ").map{Int(String($0))!}.reduce(0, +))
