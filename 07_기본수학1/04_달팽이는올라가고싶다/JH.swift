let arr = readLine()!.split(separator: " ").map{Int(String($0))!}

(arr[2]-arr[1])%(arr[0]-arr[1])==0 ? print((arr[2]-arr[1])/(arr[0]-arr[1])) : print((arr[2]-arr[1])/(arr[0]-arr[1]) + 1)
