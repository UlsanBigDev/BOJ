#include <iostream>

int main()
{
    int a,b, c;
    std::cin >> a >> b >> c;    
    
    if (a == b && a == c)
    {
        std::cout << 10000 + (a * 1000) << "\n";
    }
    else if (a == b || a == c || b == c)
    {
        if (a == b)
        {
            std::cout << 1000 + (a * 100) << "\n";
        }
        else
        {
            std::cout << 1000 + (c * 100) << "\n";
        }
    }
    else 
    {
        int max = (a > b) ? (a > c) ? a : c : (b > c) ? b : c;
        std::cout << max*100 << "\n";
    }
}
