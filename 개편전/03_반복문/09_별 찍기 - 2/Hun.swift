import Foundation

var loop = Int(readLine()!)!

for i in 1...loop {
    print(String(repeating: " ", count: loop - i) + String(repeating: "*", count: i))
}
