#include <iostream>
#include <cmath>
bool isSosu(int n)
{	
	int count = 0;
	for (int i = 1; i < n; i++) {
		if (n % i == 0) count++;
	}
	if (count == 1) return true;
	return false;
	
}
int main()
{
	int count, value = 0;
	std::cin >> count;
	for (int i=0; i<count; i++)
	{
		int n;
		std::cin >> n;
		if (isSosu(n)) value++;
	}
	std::cout << value;
	
}
