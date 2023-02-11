#include <iostream>
int main() {
    int a, count = 1, value = 1;
    std::cin >> a;
    while (value < a) {
        value += ++count;
    }		
    bool isEven = count % 2 == 0;
    int x = count;
    int y = 1;
    int temp = isEven ? value - a : a - value;		
    if (isEven) {
        while (temp > 0) {				
            x--;
            y++;
            temp--;
        }
        std::cout << x << "/" << y << "\n";
    } else {
        while (temp < 0) {
            x--;
            y++;
            temp++;
        }
        std::cout << y << "/" << x << "\n";
    }		
		
}
