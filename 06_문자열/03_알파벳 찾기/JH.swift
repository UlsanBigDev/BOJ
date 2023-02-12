var str = Array(readLine()!)
var arr = [-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1]
var num : Int = 0

for i in str{
    num = Int(i.asciiValue!-97)
    arr[num] = str.firstIndex(of: i) ?? 0
    
}

for i in arr{
    print(i, terminator: " ")
}
