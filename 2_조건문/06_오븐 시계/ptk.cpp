#include <iostream>

int main()
{
    int a,b, c;
    std::cin >> a >> b;
    std::cin >> c;
    int t = (a * 60) + b +c;
    int h = t / 60;
    if (h >= 24)
    {
        h -= 24;
    }
    int m = t % 60;    
    std::cout << h << " " << m << "\n";    

}
