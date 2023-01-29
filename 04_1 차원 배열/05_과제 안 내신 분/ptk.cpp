#include <iostream>
int main()
{
	int count, i;	
	int arr[31];
	for (i=0; i<31; i++)
	{
		arr[i] = 0;
	}

	for (i=0; i<28; i++)
	{
		int value;
		std::cin >> value;		
		arr[value] = 1;
	}	

	for (i = 1; i<31; i++)
	{
		if (arr[i] == 0)
		{
			std::cout << i << "\n";
		}
	}
}
