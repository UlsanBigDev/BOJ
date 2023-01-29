#include <iostream>
int main()
{
    int count, i ,find, result;
    int arr[100];
    std::cin >> count;
    for (i=0; i<count; i++)
    {
        std::cin >> arr[i];
    }
    std::cin >> find;
    result = 0;
    for (i=0; i<count; i++)
    {        
        if (find == arr[i])
        {
            result++;
        }
    }
    std::cout << result;
}
