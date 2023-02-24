var N = Int(readLine()!)!
var room : Int = 0
var minus : Int = 1

while N - minus > 0 {
    N = N - minus
    room += 1
    minus = 6 * room
}

N - minus <= 0 ? print(room + 1) : print(room)
