#include <iostream>
int main()
{
	int count, i;	
	int arr[10];
	int maxIndex = 0;	
	int maxValue = -1;
	for (i=0; i<9; i++)
	{
		std::cin >> arr[i];
	}	
	for (i=0; i<9; i++)
	{		
		// std::cout << arr[i] << "\n";
		if (maxValue < arr[i])
		{
			maxValue = arr[i];
			maxIndex = i;
		}
	}	
	std::cout << maxValue << "\n";
	std::cout << maxIndex + 1 << "\n";
}
