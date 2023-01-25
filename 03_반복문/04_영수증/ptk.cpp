#include <iostream>

int main()
{
    int total, totalCount, i;
    int value = 0;
    std::cin >> total >> totalCount;
    for(i=0; i<totalCount; i++) {
        int money, count, j;
        int tempTotal = 0;
        std::cin >> money >> count;
        for(j=0; j<count; j++) {
            tempTotal += money;
        }
        value += tempTotal;
    }
    if(total == value) {
        std::cout << "Yes";
    } else {
        std::cout << "No";
    }
}
