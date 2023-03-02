#include <iostream>
#include <string>
int main()
{
    int t;
    std::string str;
    std::cin >> t;
    for(int i=0; i<t; i++)
    {
        std::cin >> str;
        std::cout << str[0] << str[str.length()-1] << "\n";
    }

}
