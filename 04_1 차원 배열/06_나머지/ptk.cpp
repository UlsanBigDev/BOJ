#include <iostream>
int main()
{
	int i;
	int arr[50] = {0, };
	for (i=0; i<10; i++)
	{
		int value;
		std::cin >> value;			
		value %= 42;
		arr[value] += 1;
		
	}
	int result = 0;
	for (i=0; i<50; i++)
	{		
		if (arr[i] != 0)
		{
			result++;
		}
	}
	std::cout << result << "\n";

}
