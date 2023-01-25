var set:Set<Int> = []

for i in 1...10000 {
    set.insert(d(i))
}

for i in 1...10000 {
    if !set.contains(i) {
        print(i)
    }
}

func d(_ num:Int) -> Int {
    var sum = num
    var temp = num
    
    while temp != 0 {
        sum += temp % 10
        temp /= 10
    }
    return sum
}
