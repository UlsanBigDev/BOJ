#include <iostream>
int main()
{
	int count, i;
	int arr[1001] = { 0 };
	float m = -1;
	float total = 0;
	std::cin >> count;
	std::cout.precision(12);
	for (i=0; i<count; i++)
	{
		int value;
		std::cin >> value;
		if (m < value)
		{
			m = value;
		}
		arr[i] = value;
	}
	
	for (i=0; i<count; i++)
	{		
		float temp = arr[i] / m;		
		// std::cout << arr[i] << "|" << m << "|" << 100 << "\n";
		total += temp * 100;
	}
	float result = total / count;
	std::cout << result;

}
