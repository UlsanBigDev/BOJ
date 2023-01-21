#include <iostream>

int main()
{
    int a;
    std::cin >> a;
    for (int i=1; i<10; i++)
    {
        std::cout << a << " * " << i << " = " << a * i << "\n";
    }
}
