#include <iostream>
int main() {
    
    // 첫째 줄에 (A+B)%C, 둘째 줄에 ((A%C) + (B%C))%C, 셋째 줄에 (A×B)%C, 넷째 줄에 ((A%C) × (B%C))%C를 출력한다.
    int A,B,C;
    std::cin >> A >> B >> C;
    std::cout << (A+B)%C << "\n";
    std::cout << ((A%C) + (B%C))%C << "\n";
    std::cout << (A*B)%C << "\n";
    std::cout << ((A%C) * (B%C))%C << "\n";
    
}
