#include <iostream>

int main()
{
    int a;
    std::cin >> a;
    bool x = (a % 4 == 0) && ((a % 100) != 0 || ((a % 400) == 0));
    std::cout << x << "\n";
    
}
