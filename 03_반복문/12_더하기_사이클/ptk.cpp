#include <iostream>

int main()
{
    int a, x, y, z, d, t, c;
    std::cin >> a;   
    t = a;
    c = 0;
    do
    {
        x = t / 10; // 2
        y = t % 10; // 6
        z = (x + y) % 10;
        d = y * 10 + z;        
        t = d;
        c++;        
        
    } while (d != a);
    std::cout << c << "\n";
}
