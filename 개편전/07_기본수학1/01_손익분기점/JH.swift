let arr = readLine()!.split(separator: " ").map{Int(String($0))!}

(arr[2] > arr[1]) ? print(arr[0] / (arr[2] - arr[1]) + 1) : print(-1)
