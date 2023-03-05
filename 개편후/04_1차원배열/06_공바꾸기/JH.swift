let nums = readLine()!.split(separator: " ").map{Int(String($0))!}
var arr = [Int]()
let N = nums[0] //바구니 개수
let M = nums[1] //반복 횟수

for i in 1...N{
    arr.append(i)
}

for _ in 0..<M {
    let nums2 = readLine()!.split(separator: " ").map{Int(String($0))!}
    let first = nums2[0] - 1 //i위치
    let second = nums2[1] - 1//j위치
    let temp = arr[first]
    arr[first] = arr[second]
    arr[second] = temp
}

for i in arr{
    print(i, terminator: " ")
}
