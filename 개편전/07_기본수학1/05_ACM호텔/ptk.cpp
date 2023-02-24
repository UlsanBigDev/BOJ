#include <iostream>
#include <string>
int main()
{
	int count;
	std::cin >> count;
	for (int i = 0; i < count; i++)
	{
		int w, h, num, floor, ho;
		std::string result;
		std::cin >> h >> w >> num;
		ho = num / h + 1;
		floor = num % h; // == 0 ? h : num % h;
		if (floor == 0) 
		{
			floor = h;
			ho--;
		}

		result += std::to_string(floor);
		if (ho < 10)
		{
			result += '0' + std::to_string(ho);
		}
		else
		{
			result += std::to_string(ho);
		}

		std::cout << result << "\n";
	}
	

}
