#include <iostream>

int main()
{
    int x;
    std::cin >> x;
    for (int i=1; i<=x; i++)
    {
        for (int j=i; j>0; j--)
        {
            std::cout << "*";
        }
        std::cout << "\n";
    }
    
}
