#include <iostream>
int main()
{
	int count, i;
	int max = -1000001;
	int min = 1000000;
	std::cin >> count;
	for (i=0; i<count; i++)
	{
		int value;
		std::cin >> value;
		if (max < value)
		{
			max = value;
		}
		
		if (min > value)
		{
			min = value;
		}
	}
	std::cout << min << " " << max;
}
