#include <iostream>

int main()
{
    int a;
    std::cin.tie(NULL);
    std::ios::sync_with_stdio(false);
    std::cin >> a;
    for (int i=0; i<a; i++)
    {
        int x, y;
        std::cin >> x >> y;
        std::cout << x + y << "\n";    
    }
}
