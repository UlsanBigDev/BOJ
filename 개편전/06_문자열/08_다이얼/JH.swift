//너무 별로, switch case문 시도
let str = readLine()!.map{String($0)}
let arr2 = ["A", "B", "C"]
let arr3 = ["D", "E", "F"]
let arr4 = ["G", "H", "I"]
let arr5 = ["J", "K", "L"]
let arr6 = ["M", "N", "O"]
let arr7 = ["P", "Q", "R", "S"]
let arr8 = ["T", "U", "V"]
let arr9 = ["W", "X", "Y", "Z"]
var seconds : Int = 0
for i in str{
    if (arr2.contains(i)){
        seconds += 3
    }
    else if(arr3.contains(i)){
        seconds += 4
    }
    else if(arr4.contains(i)){
        seconds += 5
    }
    else if(arr5.contains(i)){
        seconds += 6
    }
    else if(arr6.contains(i)){
        seconds += 7
    }
    else if(arr7.contains(i)){
        seconds += 8
    }
    else if(arr8.contains(i)){
        seconds += 9
    }
    else if(arr9.contains(i)){
        seconds += 10
    }
}

print(seconds)
