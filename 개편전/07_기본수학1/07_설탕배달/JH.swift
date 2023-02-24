var N = Int(readLine()!)!
var NN : Int = 0
var x : Int = 0
var y : Int = 0

x = N % 5
y = N / 5

for i in (0...y).reversed() {
    NN = N - 5 * i
    if(NN % 3 == 0){
        print(NN/3 + i)
        break
    }
}

if (NN % 3 != 0){
    print(-1)
}
