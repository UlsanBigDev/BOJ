#include <iostream>

int main()
{
    int a, b;
    std::cin >> a >> b;
    if (b < 45)
    {
        b += 15;
        if (--a < 0)
        {
            a = 23;
        }    
    }
    else
    {
        b -= 45;    
    }
    std::cout << a << " " << b << "\n";
}
