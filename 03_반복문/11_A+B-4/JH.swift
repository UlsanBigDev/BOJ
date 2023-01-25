import Foundation

while let readString = readLine() {
    print(readString.components(separatedBy: " ").map{Int($0)!}.reduce(0,+))
}
