#include <iostream>

int main()
{
    int a, b;
    std::cin >> a >> b;
    int arr[10001];
    int result[10001];
    for (int i=0; i<a; i++)
    {
        int x;
        std::cin >> x;
        arr[i] = x;
    }
    int k = 0;
    for (int i = 0; i < a; i++)
    {
        if (arr[i] < b)
        {
            result[k] = arr[i];
            k++;
        }
    }

    for (int i=0; i<k; i++)
    {
        std::cout << result[i] << " ";
    }
}
