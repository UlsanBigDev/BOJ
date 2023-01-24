let s = readLine()!.uppercased()
var dict:[Character:Int] = [:]

for i in s {
    dict[i, default: 0] += 1
}

var max = dict.max(by: {$0.value < $1.value})!.value
var maxValue = dict.filter({ $0.value == max}).keys.sorted()
print(maxValue.count == 1 ? String(maxValue) : "?")
